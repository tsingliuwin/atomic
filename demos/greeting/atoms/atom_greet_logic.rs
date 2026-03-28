// demos/greeting/atoms/atom_greet_logic.rs

/// Atom ID: atom_greet_logic
/// Type: LOGIC
/// Description: A simple greeting logic atom.
pub fn main(name: &str) -> String {
    format!("Hello, {}! This is an Atomic greeting.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(main("Gemini"), "Hello, Gemini! This is an Atomic greeting.");
    }
}
