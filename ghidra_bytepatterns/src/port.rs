//! The Rust port of `ghidra/bitpatterns/info/DataGatheringParams.java`.
//!
//! Replace each `unimplemented!` with the port of the source function. Return `Err` where
//! the source throws or raises: the caller gets an error there too. After changing a
//! signature, run `rust-anything refresh` to regenerate the bindings. `rust-anything diff` checks
//! behavior against what the source recorded.

#![forbid(unsafe_code)]

/// Port of `getContextRegisterList` (`ghidra/bitpatterns/info/DataGatheringParams.java:168`).
pub fn get_context_register_list(context_regs_csv: String) -> Result<Vec<String>, String> {
    let mut context_registers = Vec::new();

    if context_regs_csv.is_empty() {
        return Ok(context_registers);
    }

    let trimmed = context_regs_csv.trim();
    // literal string null: this can be a value in a .properties file
    if trimmed.is_empty() || trimmed == "null" {
        return Ok(context_registers);
    }

    let components = trimmed.split(",");
    for component in components {
        let trimmed_component = component.trim();
        if !trimmed_component.is_empty() {
            context_registers.push(trimmed_component.to_string());
        }
    }

    Ok(context_registers)
}
