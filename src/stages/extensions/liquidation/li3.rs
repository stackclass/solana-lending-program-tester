use crate::verifier::get_program_info;

pub fn test_liquidation_process(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_liquidate_fn =
        info.instructions.iter().any(|inst| inst.name.to_lowercase().contains("liquidate"));
    if has_liquidate_fn {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Liquidation function not found".to_string())))
    }
}
