use crate::verifier::get_program_info;

pub fn test_bank_account(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_bank = info.structs.iter().any(|s| s.name.to_lowercase().contains("bank")) ||
        info.accounts.iter().any(|acc| acc.name.to_lowercase().contains("bank"));
    if has_bank {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Bank account not found".to_string())))
    }
}
