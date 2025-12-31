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

use crate::verifier::get_program_info;

pub fn test_core_practice(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_lending_fn = info
        .instructions
        .iter()
        .filter(|inst| {
            inst.name.to_lowercase().contains("borrow") ||
                inst.name.to_lowercase().contains("repay") ||
                inst.name.to_lowercase().contains("lend")
        })
        .count() >=
        2;
    if has_lending_fn {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Core lending functions not found".to_string())))
    }
}
