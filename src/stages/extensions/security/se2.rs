use crate::verifier::get_program_info;

pub fn test_reentrancy_protection(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_reentrancy = info
        .accounts
        .iter()
        .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("reentrancy")));
    if has_reentrancy {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Reentrancy protection not found".to_string())))
    }
}
