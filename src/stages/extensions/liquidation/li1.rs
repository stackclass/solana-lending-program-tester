use crate::verifier::get_program_info;

pub fn test_health_factor(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_health = info
        .structs
        .iter()
        .any(|s| s.fields.iter().any(|f| f.name.to_lowercase().contains("health"))) ||
        info.accounts
            .iter()
            .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("health")));
    if has_health {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Health factor not found".to_string())))
    }
}
