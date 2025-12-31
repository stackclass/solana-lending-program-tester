use crate::verifier::get_program_info;

pub fn test_liquidation_trigger(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_liquidate_trigger = info.structs.iter().any(|s| {
        s.fields.iter().any(|f| {
            f.name.to_lowercase().contains("health") || f.name.to_lowercase().contains("threshold")
        })
    });
    if has_liquidate_trigger {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Liquidation trigger not found".to_string())))
    }
}
