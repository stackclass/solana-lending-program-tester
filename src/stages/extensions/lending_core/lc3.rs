use crate::verifier::get_program_info;

pub fn test_ltv_calculation(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_ltv = info.structs.iter().any(|s| {
        s.fields.iter().any(|f| {
            f.name.to_lowercase().contains("ltv") || f.name.to_lowercase().contains("collateral")
        })
    }) || info
        .accounts
        .iter()
        .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("ltv")));
    if has_ltv {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("LTV/health factor not found".to_string())))
    }
}
