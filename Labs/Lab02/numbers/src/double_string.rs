pub struct DoubleString(String, String);

impl DoubleString {
    pub fn from_strs(s1: &str, s2: &str) -> Self {
        Self(s1.to_string(), s2.to_string())
    }

    pub fn from_strings(s1: &String, s2: &String) -> Self {
        Self(s1.clone(), s2.clone())
    }

    pub fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}