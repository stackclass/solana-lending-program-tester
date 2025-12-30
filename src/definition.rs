use std::sync::Arc;

use tester::{Case, Definition};

use crate::stages::{
    base::*,
    extensions::{
        account_structure::{as1, as2, as3, as4},
        interest::{in1, in2, in3, in4},
        lending_core::{lc1, lc2, lc3, lc4},
        liquidation::{li1, li2, li3, li4, li5},
        oracle::{or1, or2, or3, or4},
        pda::{pa1, pa2, pa3, pa4},
        security::{se1, se2, se3, se4},
        treasury::{tr1, tr2, tr3, tr4},
    },
};

pub fn build() -> Definition {
    Definition {
        executable_name: "your_program.sh".to_string(),
        legacy_executable_name: Some("your_program.sh".to_string()),
        cases: vec![
            Case::new("be1", Arc::new(be1::test_env_setup)),
            Case::new("rs2", Arc::new(rs2::test_rust_basics)),
            Case::new("sm3", Arc::new(sm3::test_solana_model)),
            Case::new("at4", Arc::new(at4::test_anchor_try)),
            Case::new("st5", Arc::new(st5::test_spl_token_basics)),
            Case::new("cp6", Arc::new(cp6::test_basic_deposit)),
            Case::new("tt7", Arc::new(tt7::test_basic_withdraw)),
            Case::new("pa1", Arc::new(pa1::test_pda_concept)),
            Case::new("pa2", Arc::new(pa2::test_pda_derivation)),
            Case::new("pa3", Arc::new(pa3::test_pda_bump_seeds)),
            Case::new("pa4", Arc::new(pa4::test_pda_practice)),
            Case::new("tr1", Arc::new(tr1::test_treasury_intro)),
            Case::new("tr2", Arc::new(tr2::test_treasury_creation)),
            Case::new("tr3", Arc::new(tr3::test_treasury_security)),
            Case::new("tr4", Arc::new(tr4::test_treasury_practice)),
            Case::new("as1", Arc::new(as1::test_bank_account)),
            Case::new("as2", Arc::new(as2::test_user_account)),
            Case::new("as3", Arc::new(as3::test_account_space)),
            Case::new("as4", Arc::new(as4::test_account_practice)),
            Case::new("lc1", Arc::new(lc1::test_borrow_basics)),
            Case::new("lc2", Arc::new(lc2::test_repay_basics)),
            Case::new("lc3", Arc::new(lc3::test_ltv_calculation)),
            Case::new("lc4", Arc::new(lc4::test_core_practice)),
            Case::new("or1", Arc::new(or1::test_oracle_concept)),
            Case::new("or2", Arc::new(or2::test_pyth_integration)),
            Case::new("or3", Arc::new(or3::test_price_fetching)),
            Case::new("or4", Arc::new(or4::test_oracle_practice)),
            Case::new("li1", Arc::new(li1::test_health_factor)),
            Case::new("li2", Arc::new(li2::test_liquidation_trigger)),
            Case::new("li3", Arc::new(li3::test_liquidation_process)),
            Case::new("li4", Arc::new(li4::test_liquidation_bonus)),
            Case::new("li5", Arc::new(li5::test_liquidation_practice)),
            Case::new("in1", Arc::new(in1::test_interest_basics)),
            Case::new("in2", Arc::new(in2::test_accrued_interest)),
            Case::new("in3", Arc::new(in3::test_rate_models)),
            Case::new("in4", Arc::new(in4::test_interest_practice)),
            Case::new("se1", Arc::new(se1::test_common_vulnerabilities)),
            Case::new("se2", Arc::new(se2::test_reentrancy_protection)),
            Case::new("se3", Arc::new(se3::test_account_validation)),
            Case::new("se4", Arc::new(se4::test_security_practice)),
        ],
        ..Default::default()
    }
}
