/// Simple data holder for ContextRegisterInfo that can be bridged to Rust.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SimpleContextRegisterInfo {
    /// The context register
    pub context_register: Option<String>,
    /// The value as string
    pub value_as_string: Option<String>,
}

impl SimpleContextRegisterInfo {
    /// Creates a new SimpleContextRegisterInfo object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new SimpleContextRegisterInfo object with the given context register and value as string.
    ///
    /// # Arguments
    ///
    /// * `context_register` - The context register (can be null)
    /// * `value_as_string` - The value as string (can be null)
    pub fn new_with_values(context_register: Option<String>, value_as_string: Option<String>) -> Self {
        Self {
            context_register,
            value_as_string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_context_register_info() {
        // Test default constructor
        let info = SimpleContextRegisterInfo::new();
        assert_eq!(info.context_register, None);
        assert_eq!(info.value_as_string, None);

        // Test constructor with values
        let info = SimpleContextRegisterInfo::new_with_values(Some("REG".to_string()), Some("42".to_string()));
        assert_eq!(info.context_register, Some("REG".to_string()));
        assert_eq!(info.value_as_string, Some("42".to_string()));

        // Test constructor with null values
        let info = SimpleContextRegisterInfo::new_with_values(None, None);
        assert_eq!(info.context_register, None);
        assert_eq!(info.value_as_string, None);

        // Test constructor with one null
        let info = SimpleContextRegisterInfo::new_with_values(Some("REG".to_string()), None);
        assert_eq!(info.context_register, Some("REG".to_string()));
        assert_eq!(info.value_as_string, None);
    }
}