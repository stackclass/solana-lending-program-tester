use crate::verifier::get_program_info;

pub fn test_treasury_creation(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_treasury_init =
        info.accounts.iter().any(|acc| acc.name.to_lowercase().contains("treasury"));
    if has_treasury_init {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Treasury creation not found".to_string())))
    }
}
