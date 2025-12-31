use crate::verifier::get_program_info;

pub fn test_pda_practice(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_pda_practice = info.accounts.iter().any(|acc| acc.name.to_lowercase().contains("pda")) ||
        info.instructions.iter().any(|inst| inst.name.to_lowercase().contains("pda"));
    if has_pda_practice {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("PDA practice not found".to_string())))
    }
}
