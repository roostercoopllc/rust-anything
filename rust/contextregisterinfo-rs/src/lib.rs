use num_bigint::BigInt;
use roxmltree::Document;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// ContextRegisterInfo represents the values a specific context register assumes within a function body.
#[derive(Debug, PartialEq, Clone)]
pub struct ContextRegisterInfo {
    /// The context register
    context_register: Option<String>,
    /// The value it assumes
    value: Option<BigInt>,
}

impl Default for ContextRegisterInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextRegisterInfo {
    /// Default constructor (used by XMLEncoder)
    pub fn new() -> Self {
        Self {
            context_register: None,
            value: None,
        }
    }

    /// Creates a {@link ContextRegisterInfo} object for a specified context register
    pub fn with_context_register(context_register: String) -> Self {
        Self {
            context_register: Some(context_register),
            value: None,
        }
    }

    /// Returns the context register associated with this
    pub fn get_context_register(&self) -> Option<String> {
        self.context_register.clone()
    }

    /// Sets the context register associated with this
    pub fn set_context_register(&mut self, context_register: String) {
        self.context_register = Some(context_register);
    }

    /// Sets the value associated with this
    pub fn set_value(&mut self, value: BigInt) {
        self.value = Some(value);
    }

    /// Returns the value associated with this
    pub fn get_value(&self) -> Option<BigInt> {
        self.value.clone()
    }

    /// Indicates whether some other object is "equal to" this one.
    pub fn equals(&self, other: &Self) -> bool {
        self.context_register == other.context_register && self.value == other.value
    }

    /// Returns a hash code value for the object.
    pub fn hash_code(&self) -> i32 {
        let prime: i32 = 31;
        let mut result: i32 = 1;
        result = prime.wrapping_mul(result)
            + self.context_register.as_ref().map_or(0, |s| {
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                hasher.finish() as i32
            });
        result = prime.wrapping_mul(result)
            + self.value.as_ref().map_or(0, |v| {
                let mut hasher = DefaultHasher::new();
                v.hash(&mut hasher);
                hasher.finish() as i32
            });
        result
    }

    /// Creates a {@link ContextRegisterInfo} object using data in the supplied XML node.
    pub fn from_xml(xml: &str) -> Self {
        let mut context_register = None;
        let mut value = None;

        if let Ok(doc) = Document::parse(xml) {
            for node in doc.descendants() {
                if node.has_tag_name("ContextRegisterInfo") {
                    if let Some(attr) = node.attribute("contextRegister") {
                        context_register = Some(attr.to_string());
                    }
                    if let Some(attr) = node.attribute("value") {
                        if let Some(val) = BigInt::parse_bytes(attr.as_bytes(), 10) {
                            value = Some(val);
                        }
                    }
                    break;
                }
            }
        }

        Self {
            context_register,
            value,
        }
    }

    /// Converts this object into XML
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<ContextRegisterInfo");
        if let Some(ref cr) = self.context_register {
            s.push_str(&format!(" contextRegister=\"{}\"", escape_xml(cr)));
        }
        if let Some(ref val) = self.value {
            s.push_str(&format!(" value=\"{}\"", val));
        }
        s.push_str("/>");
        s
    }
}

impl fmt::Display for ContextRegisterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.context_register, &self.value) {
            (Some(cr), Some(val)) => write!(f, "{}={}", cr, val),
            (Some(cr), None) => write!(f, "{}=", cr),
            (None, Some(val)) => write!(f, "null={}", val),
            (None, None) => write!(f, "null=null"),
        }
    }
}

/// Static methods matching the Java implementation
impl ContextRegisterInfo {
    /// Returns the context register associated with the instance
    pub fn get_context_register_static(instance: &Option<ContextRegisterInfo>) -> Option<String> {
        instance.as_ref().and_then(|i| i.context_register.clone())
    }

    /// Sets the context register associated with the instance
    pub fn set_context_register_static(
        instance: &mut Option<ContextRegisterInfo>,
        context_register: String,
    ) {
        if let Some(instance) = instance {
            instance.context_register = Some(context_register);
        }
    }

    /// Sets the value associated with the instance
    pub fn set_value_static(instance: &mut Option<ContextRegisterInfo>, value: BigInt) {
        if let Some(instance) = instance {
            instance.value = Some(value);
        }
    }

    /// Returns the value associated with the instance
    pub fn get_value_static(instance: &Option<ContextRegisterInfo>) -> Option<BigInt> {
        instance.as_ref().and_then(|i| i.value.clone())
    }

    /// Returns the string representation of the instance
    pub fn to_string_static(instance: &Option<ContextRegisterInfo>) -> String {
        match instance {
            Some(info) => info.to_string(),
            None => "null".to_string(),
        }
    }

    /// Indicates whether some other object is "equal to" this one.
    pub fn equals_static(
        instance: &Option<ContextRegisterInfo>,
        obj: &Option<ContextRegisterInfo>,
    ) -> bool {
        match (instance, obj) {
            (None, None) => true,
            (Some(i), Some(o)) => i.equals(o),
            _ => false,
        }
    }

    /// Returns a hash code value for the object.
    pub fn hash_code_static(instance: &Option<ContextRegisterInfo>) -> i32 {
        match instance {
            Some(info) => info.hash_code(),
            None => 0,
        }
    }

    /// Creates a {@link ContextRegisterInfo} object using data in the supplied XML node.
    pub fn from_xml_static(xml: &str) -> ContextRegisterInfo {
        ContextRegisterInfo::from_xml(xml)
    }

    /// Converts this object into XML
    pub fn to_xml_static(instance: &ContextRegisterInfo) -> String {
        instance.to_xml()
    }
}

/// Escape XML special characters in a string.
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_register_info() {
        // Test default constructor
        let info = ContextRegisterInfo::new();
        assert_eq!(info.get_context_register(), None);
        assert_eq!(info.get_value(), None);

        // Test constructor with context register
        let info = ContextRegisterInfo::with_context_register("REG".to_string());
        assert_eq!(info.get_context_register(), Some("REG".to_string()));
        assert_eq!(info.get_value(), None);

        // Test setters and getters
        let mut info = ContextRegisterInfo::new();
        info.set_context_register("REG".to_string());
        assert_eq!(info.get_context_register(), Some("REG".to_string()));

        let val = BigInt::from(42i32);
        info.set_value(val.clone());
        assert_eq!(info.get_value(), Some(val));

        // Test toString
        let info = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        assert_eq!(format!("{}", info), "REG=42");

        // Test equals
        let info1 = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        let info2 = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        assert!(info1.equals(&info2));

        let info3 = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(43i32)),
        };
        assert!(!info1.equals(&info3));

        // Test hashCode
        let info1 = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        let info2 = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        assert_eq!(info1.hash_code(), info2.hash_code());

        // Test XML
        let info = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        let xml = info.to_xml();
        assert_eq!(
            xml,
            "<ContextRegisterInfo contextRegister=\"REG\" value=\"42\"/>"
        );

        let info2 = ContextRegisterInfo::from_xml(&xml);
        assert_eq!(info2.get_context_register(), Some("REG".to_string()));
        assert_eq!(info2.get_value(), Some(BigInt::from(42i32)));

        // Test XML with only context_register
        let info = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: None,
        };
        let xml = info.to_xml();
        assert_eq!(xml, "<ContextRegisterInfo contextRegister=\"REG\"/>");

        let info2 = ContextRegisterInfo::from_xml(&xml);
        assert_eq!(info2.get_context_register(), Some("REG".to_string()));
        assert_eq!(info2.get_value(), None);

        // Test XML with only value
        let info = ContextRegisterInfo {
            context_register: None,
            value: Some(BigInt::from(42i32)),
        };
        let xml = info.to_xml();
        assert_eq!(xml, "<ContextRegisterInfo value=\"42\"/>");

        let info2 = ContextRegisterInfo::from_xml(&xml);
        assert_eq!(info2.get_context_register(), None);
        assert_eq!(info2.get_value(), Some(BigInt::from(42i32)));

        // Test XML with both null
        let info = ContextRegisterInfo {
            context_register: None,
            value: None,
        };
        let xml = info.to_xml();
        assert_eq!(xml, "<ContextRegisterInfo/>");

        let info2 = ContextRegisterInfo::from_xml(&xml);
        assert_eq!(info2.get_context_register(), None);
        assert_eq!(info2.get_value(), None);
    }

    #[test]
    fn test_static_methods() {
        // Test get_context_register_static
        assert_eq!(
            ContextRegisterInfo::get_context_register_static(&None),
            None
        );
        let info = ContextRegisterInfo::with_context_register("REG".to_string());
        assert_eq!(
            ContextRegisterInfo::get_context_register_static(&Some(info)),
            Some("REG".to_string())
        );

        // Test set_context_register_static
        let mut instance = None;
        ContextRegisterInfo::set_context_register_static(&mut instance, "REG".to_string());
        assert_eq!(instance, None); // Passing None does nothing
        let mut instance = Some(ContextRegisterInfo::new());
        ContextRegisterInfo::set_context_register_static(&mut instance, "REG2".to_string());
        assert_eq!(
            instance.as_ref().unwrap().get_context_register(),
            Some("REG2".to_string())
        );

        // Test get_value_static
        assert_eq!(ContextRegisterInfo::get_value_static(&None), None);
        let mut info = ContextRegisterInfo::with_context_register("REG".to_string());
        info.set_value(BigInt::from(42i32));
        assert_eq!(
            ContextRegisterInfo::get_value_static(&Some(info)),
            Some(BigInt::from(42i32))
        );

        // Test set_value_static
        let mut instance = None;
        ContextRegisterInfo::set_value_static(&mut instance, BigInt::from(42i32));
        assert_eq!(instance, None); // Passing None does nothing
        let mut instance = Some(ContextRegisterInfo::new());
        ContextRegisterInfo::set_value_static(&mut instance, BigInt::from(43i32));
        assert_eq!(
            instance.as_ref().unwrap().get_value(),
            Some(BigInt::from(43i32))
        );

        // Test to_string_static
        assert_eq!(
            ContextRegisterInfo::to_string_static(&None),
            "null".to_string()
        );
        let info = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        assert_eq!(
            ContextRegisterInfo::to_string_static(&Some(info)),
            "REG=42".to_string()
        );

        // Test equals_static
        assert!(ContextRegisterInfo::equals_static(&None, &None));
        assert!(!ContextRegisterInfo::equals_static(
            &None,
            &Some(ContextRegisterInfo::new())
        ));
        assert!(!ContextRegisterInfo::equals_static(
            &Some(ContextRegisterInfo::new()),
            &None
        ));
        let mut info1 = ContextRegisterInfo::with_context_register("REG".to_string());
        info1.set_value(BigInt::from(42i32));
        let mut info2 = ContextRegisterInfo::with_context_register("REG".to_string());
        info2.set_value(BigInt::from(42i32));
        assert!(ContextRegisterInfo::equals_static(
            &Some(info1.clone()),
            &Some(info2.clone())
        ));
        let mut info3 = ContextRegisterInfo::with_context_register("REG".to_string());
        info3.set_value(BigInt::from(43i32));
        assert!(!ContextRegisterInfo::equals_static(
            &Some(info1.clone()),
            &Some(info3.clone())
        ));

        // Test hash_code_static
        assert_eq!(ContextRegisterInfo::hash_code_static(&None), 0);
        let info = ContextRegisterInfo {
            context_register: Some("REG".to_string()),
            value: Some(BigInt::from(42i32)),
        };
        let expected = info.hash_code();
        assert_eq!(ContextRegisterInfo::hash_code_static(&Some(info)), expected);
    }
}
