use crate::verifier::get_program_info;

pub fn test_account_space(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_space = info
        .structs
        .iter()
        .any(|s| s.fields.iter().any(|f| f.name.to_lowercase().contains("space")));
    if has_space {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Account space not found".to_string())))
    }
}
