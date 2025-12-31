use crate::verifier::get_program_info;

pub fn test_pda_bump_seeds(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_bump = info
        .accounts
        .iter()
        .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("bump")));
    if has_bump {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Bump seeds not found".to_string())))
    }
}
