// Copyright (c) The StackClass Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub details: serde_json::Value,
}

#[derive(Debug)]
struct VerificationError {
    message: String,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VerificationError {}

pub fn run_verification(
    _harness: &tester::Harness,
    stage_id: &str,
) -> Result<(), tester::CaseError> {
    let repository_dir = std::env::var("STACKCLASS_REPOSITORY_DIR").map_err(|_| {
        Box::new(VerificationError { message: "STACKCLASS_REPOSITORY_DIR not set".to_string() })
    })?;

    let executable_path = PathBuf::from(&repository_dir).join("your_program.sh");

    let mut cmd = Command::new(&executable_path);
    cmd.arg("--check").arg(stage_id);

    let output = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            Box::new(VerificationError { message: format!("Failed to run verification: {}", e) })
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(VerificationError {
            message: format!("Verification command failed: {}", stderr),
        }));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let result: VerificationResult = serde_json::from_str(&stdout).map_err(|e| {
        Box::new(VerificationError { message: format!("Failed to parse JSON: {} - {}", e, stdout) })
    })?;

    if result.success {
        Ok(())
    } else {
        Err(Box::new(VerificationError { message: result.message }))
    }
}
