use std::string::String;

pub trait Manip {
    fn remove_last_char(string_a: &str) -> String;
    fn reverser(string_a: &str) -> String;
    fn size(string_a: &str) -> usize;
}

impl Manip for String {
    /// Removes last character from the &str
    ///
    /// # Arguments
    /// * `string_a` a string slice
    ///
    fn remove_last_char(string_a: &str) -> String {
        if string_a.len() == 0 { return string_a.to_string()}
        string_a.to_string()[..string_a.chars().count() - 1].to_string()
    }

    /// Reverses the &str
    ///
    /// # Arguments
    /// * `string_a` a string slice
    ///
    fn reverser(string_a: &str) -> String {
        let mut return_string = String::new();
        for char in string_a.chars() {
            return_string.insert(0, char)
        }
        return_string
    }

    /// Returns a usize of the character count in the &str
    ///
    /// # Arguments
    /// * `string_a` a string slice
    ///
    fn size(string_a: &str) -> usize {
        string_a.chars().count()
    }
}

#[cfg(test)]
mod tests {
    use ::Manip;
    #[test]
    fn remove_last_char() {
        let my_string = "hello";
        assert_eq!("hell", String::remove_last_char(&my_string))
    }

    #[test]
    fn remove_last_char_empty_str() {
        let empty_string = "";
        assert_eq!("", String::remove_last_char(&empty_string))
    }

    #[test]
    fn reverser() {
        let my_string = "hello";
        assert_eq!("olleh", String::reverser(&my_string))
    }

    #[test]
    fn reverser_empty_str() {
        let empty_string = "";
        assert_eq!("", String::remove_last_char(&empty_string))
    }

    #[test]
    fn size() {
        let my_string = "hello";
        assert_eq!(5, String::size(&my_string))
    }

    #[test]
    fn size_empty_str() {
        let empty_string = "";
        assert_eq!(0, String::size(&empty_string))
    }
}
