

/// DataGatheringParams is a container for the parameters used when collecting function start data to be mined.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DataGatheringParams {
    /// Number of prebytes (bytes before a function start)
    num_pre_bytes: i32,
    /// Number of first bytes (bytes immediately after and including a function start)
    num_first_bytes: i32,
    /// Number of return bytes (bytes immediately before and including a return instruction)
    num_return_bytes: i32,
    /// Number of pre instructions (instructions immediately before a function start)
    num_pre_instructions: i32,
    /// Number of first instructions (instructions immediately after and including a function start)
    num_first_instructions: i32,
    /// Number of return instructions (instructions immediately before and including a return instruction)
    num_return_instructions: i32,
    /// List of context registers tracked during data gathering
    context_registers: Vec<String>,
}

impl DataGatheringParams {
    /// Creates a new DataGatheringParams object. Use setter methods to set the values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the number of prebytes, i.e., bytes before a function start.
    /// @return number of prebytes
    pub fn get_num_pre_bytes(&self) -> i32 {
        self.num_pre_bytes
    }

    /// Set the number of prebytes.
    /// @param pre_bytes number of prebytes
    pub fn set_num_pre_bytes(&mut self, pre_bytes: i32) {
        self.num_pre_bytes = pre_bytes;
    }

    /// Get the number of first bytes, i.e., the number of bytes immediately after (and including)
    /// a function start.
    /// @return number of first bytes
    pub fn get_num_first_bytes(&self) -> i32 {
        self.num_first_bytes
    }

    /// Set the number of first bytes
    /// @param first_bytes the number of first bytes
    pub fn set_num_first_bytes(&mut self, first_bytes: i32) {
        self.num_first_bytes = first_bytes;
    }

    /// Get the number of return bytes, i.e., the number of bytes immediately before (and including)
    /// a return instruction.
    /// @return number of return bytes
    pub fn get_num_return_bytes(&self) -> i32 {
        self.num_return_bytes
    }

    /// Set the number of return bytes.
    /// @param return_bytes number of return bytes
    pub fn set_num_return_bytes(&mut self, return_bytes: i32) {
        self.num_return_bytes = return_bytes;
    }

    /// Get the number of pre Instructions, i.e., the number of instructions immediately before
    /// a function start
    /// @return number of pre Instructions
    pub fn get_num_pre_instructions(&self) -> i32 {
        self.num_pre_instructions
    }

    /// Set the number of pre Instructions
    /// @param pre_instructions number of pre Instructions
    pub fn set_num_pre_instructions(&mut self, pre_instructions: i32) {
        self.num_pre_instructions = pre_instructions;
    }

    /// Get the number of first Instructions, i.e., the number of instructions immediately after
    /// (and including) a function start
    /// @return number of first instructions
    pub fn get_num_first_instructions(&self) -> i32 {
        self.num_first_instructions
    }

    /// Set the number of first instructions to collect during data gathering
    /// @param first_instructions number of first instructions
    pub fn set_num_first_instructions(&mut self, first_instructions: i32) {
        self.num_first_instructions = first_instructions;
    }

    /// Get the number of return instructions, i.e., the number of instructions immediately before
    /// (and including) a return instruction
    /// @return number of return instructions to gather
    pub fn get_num_return_instructions(&self) -> i32 {
        self.num_return_instructions
    }

    /// Set the number of return instructions.
    /// @param return_instructions number of return instructions to gather
    pub fn set_num_return_instructions(&mut self, return_instructions: i32) {
        self.num_return_instructions = return_instructions;
    }

    /// Get a list of context registers tracked during data gathering
    /// @return list of context registers
    pub fn get_context_registers(&self) -> Vec<String> {
        self.context_registers.clone()
    }

    /// Set the context registers to track during data gathering
    /// @param c_regs registers to track
    pub fn set_context_registers(&mut self, c_regs: Vec<String>) {
        self.context_registers = c_regs;
    }

    /// Parse a List of context registers from a CSV string of context register names
    /// 
    /// It is assumed that the list contains no duplicates.
    /// 
    /// @param context_regs_csv a CSV String of context register names to track during
    /// data gathering
    ///
    /// @return the list
    pub fn get_context_register_list(context_regs_csv: &str) -> Vec<String> {
        let mut context_registers = Vec::new();

        if context_regs_csv.is_empty() {
            return context_registers;
        }

        let trimmed = context_regs_csv.trim();
        // literal string null: this can be a value in a .properties file
        if trimmed.is_empty() || trimmed == "null" {
            return context_registers;
        }

        let components = trimmed.split(',');
        for component in components {
            let trimmed_component = component.trim();
            if !trimmed_component.is_empty() && trimmed_component != "null" {
                context_registers.push(trimmed_component.to_string());
            }
        }
        context_registers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_gathering_params() {
        // Test default constructor
        let params = DataGatheringParams::new();
        assert_eq!(params.get_num_pre_bytes(), 0);
        assert_eq!(params.get_num_first_bytes(), 0);
        assert_eq!(params.get_num_return_bytes(), 0);
        assert_eq!(params.get_num_pre_instructions(), 0);
        assert_eq!(params.get_num_first_instructions(), 0);
        assert_eq!(params.get_num_return_instructions(), 0);
        assert!(params.get_context_registers().is_empty());

        // Test setters and getters
        let mut params = DataGatheringParams::new();
        params.set_num_pre_bytes(1);
        assert_eq!(params.get_num_pre_bytes(), 1);

        params.set_num_first_bytes(2);
        assert_eq!(params.get_num_first_bytes(), 2);

        params.set_num_return_bytes(3);
        assert_eq!(params.get_num_return_bytes(), 3);

        params.set_num_pre_instructions(4);
        assert_eq!(params.get_num_pre_instructions(), 4);

        params.set_num_first_instructions(5);
        assert_eq!(params.get_num_first_instructions(), 5);

        params.set_num_return_instructions(6);
        assert_eq!(params.get_num_return_instructions(), 6);

        // Test context registers
        let mut params = DataGatheringParams::new();
        let regs = vec!["REG1".to_string(), "REG2".to_string()];
        params.set_context_registers(regs.clone());
        assert_eq!(params.get_context_registers(), regs);

        // Test get_context_register_list
        let result = DataGatheringParams::get_context_register_list("REG1,REG2,REG3");
        assert_eq!(result, vec!["REG1", "REG2", "REG3"]);

        // Test empty string
        let result = DataGatheringParams::get_context_register_list("");
        assert!(result.is_empty());

        // Test null string
        let result = DataGatheringParams::get_context_register_list("null");
        assert!(result.is_empty());

        // Test whitespace
        let result = DataGatheringParams::get_context_register_list("  REG1  ,  REG2  ");
        assert_eq!(result, vec!["REG1", "REG2"]);

        // Test mixed empty and null
        let result = DataGatheringParams::get_context_register_list(",null,,REG1,");
        assert_eq!(result, vec!["REG1"]);
    }
}