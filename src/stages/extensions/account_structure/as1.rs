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

use crate::verifier::run_verification;

pub fn test_bank_account(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as1")
}

pub fn test_user_account(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as2")
}

pub fn test_account_space(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as3")
}

pub fn test_account_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as4")
}
