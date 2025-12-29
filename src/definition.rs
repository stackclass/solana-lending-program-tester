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

use std::sync::Arc;

use tester::{Case, Definition};

use crate::stages::{advanced_features::*, base::*};

pub fn build() -> Definition {
    Definition {
        executable_name: "your_program.sh".to_string(),
        legacy_executable_name: Some("your_program.sh".to_string()),
        cases: vec![
            // Base Stages
            Case::new("ry1", Arc::new(ry1::test_declare_id)),
            Case::new("bs2", Arc::new(bs2::test_bank)),
            Case::new("us3", Arc::new(us3::test_user)),
            Case::new("ib4", Arc::new(ib4::test_init_bank)),
            Case::new("iu5", Arc::new(iu5::test_init_user)),
            Case::new("dp6", Arc::new(dp6::test_deposit)),
            Case::new("wt7", Arc::new(wt7::test_withdraw)),
            Case::new("br8", Arc::new(br8::test_borrow)),
            Case::new("rp9", Arc::new(rp9::test_repay)),
            Case::new("lq10", Arc::new(lq10::test_liquidate)),
            Case::new("er11", Arc::new(er11::test_error_codes)),
            Case::new("py12", Arc::new(py12::test_pyth_integration)),
            Case::new("td13", Arc::new(td13::test_testing_deploy)),
            // Advanced Features
            Case::new("ir1", Arc::new(ir1::test_interest_rate_model)),
            Case::new("fl2", Arc::new(fl2::test_flash_loans)),
        ],
        ..Default::default()
    }
}
