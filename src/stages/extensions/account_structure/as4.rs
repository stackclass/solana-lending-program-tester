use crate::verifier::get_program_info;

pub fn test_account_practice(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    if info.accounts.len() > 1 || !info.structs.is_empty() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Account implementation not found".to_string())))
    }
}
