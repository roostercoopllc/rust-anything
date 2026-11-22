/// Objects of this class are used to access prefixes or suffixes of strings, as well as filter strings by length.
#[derive(Debug, PartialEq, Clone)]
pub struct ByteSequenceLengthFilter {
    /// if positive, the filter will return the first internalIndex characters
    /// if negative, it will return the last internalIndex characters (think python string slicing)
    internal_index: i32,
    /// the minimum length a string needs to be
    min_length: i32,
}

impl ByteSequenceLengthFilter {
    /// @param internal_index if positive, filter will return the first internal_index characters
    /// in the string. Otherwise it will return the last internal_index characters.
    /// @param min_length the minimum length of a string
    /// @throws IllegalArgumentException if min_length is negative or min_length < internal_index.abs()
    pub fn new(internal_index: i32, min_length: i32) -> Self {
        if min_length < 0 {
            panic!("minLength must be non-negative!");
        }

        if min_length < internal_index.abs() {
            panic!("minLength too small for this internalIndex!");
        }
        //strings are hex digits, want to filter on the number of bytes
        Self {
            internal_index: 2 * internal_index,
            min_length: 2 * min_length,
        }
    }

    /// Applies filter to base
    /// @param base the String to filter
    /// @return The filtered String, or None if base does not meet the minimum length requirement.
    pub fn filter(&self, base: &str) -> Option<String> {
        if base.is_empty() || base.len() < self.min_length as usize {
            return None;
        }
        if self.internal_index >= 0 {
            Some(base[..self.internal_index as usize].to_string())
        } else {
            let len = base.len() as i64;
            let start = (len + self.internal_index as i64) as usize;
            let end = len as usize;
            Some(base[start..end].to_string())
        }
    }
}

use std::fmt;
impl fmt::Display for ByteSequenceLengthFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "internalIndex: {}\nminLength: {}\n", self.internal_index, self.min_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_sequence_length_filter() {
        // Test normal case - positive internalIndex
        let filter = ByteSequenceLengthFilter::new(2, 4); // internalIndex=4, minLength=8
        assert_eq!(filter.filter("abcdefgh"), Some("abcd".to_string())); // First 2 bytes (4 hex chars)
        assert_eq!(filter.filter("abcdef"), None); // Too short
        
        // Test negative internalIndex
        let filter = ByteSequenceLengthFilter::new(-2, 4); // internalIndex=-4, minLength=8
        assert_eq!(filter.filter("abcdefgh"), Some("efgh".to_string())); // Last 2 bytes (4 hex chars)
        assert_eq!(filter.filter("abcdef"), None); // Too short
        
        // Test zero internalIndex
        let filter = ByteSequenceLengthFilter::new(0, 4); // internalIndex=0, minLength=8
        assert_eq!(filter.filter("abcdefgh"), Some("".to_string())); // First 0 bytes
        assert_eq!(filter.filter("abcdef"), None); // Too short
        
        // Test toString
        let filter = ByteSequenceLengthFilter::new(2, 4);
        assert_eq!(filter.to_string(), "internalIndex: 4\nminLength: 8\n");
        
        // Test edge cases
        let filter = ByteSequenceLengthFilter::new(3, 3); // internalIndex=6, minLength=6
        assert_eq!(filter.filter("abcdef"), Some("abcdef".to_string())); // Exactly min length
        assert_eq!(filter.filter("abcde"), None); // Too short
    }
    
    #[test]
    #[should_panic(expected = "minLength must be non-negative!")]
    fn test_negative_min_length() {
        ByteSequenceLengthFilter::new(2, -1);
    }
    
    #[test]
    #[should_panic(expected = "minLength too small for this internalIndex!")]
    fn test_min_length_too_small() {
        ByteSequenceLengthFilter::new(5, 4); // internalIndex=10, minLength=8
    }
}