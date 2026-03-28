use std::env;

/// Atom ID: atom_greet_logic
/// Type: LOGIC
/// Description: A simple greeting logic atom.
pub fn greet(name: &str) -> String {
    format!("Hello, {}! This is an Atomic greeting.", name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = if args.len() > 1 { &args[1] } else { "Stranger" };
    println!("{}", greet(name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Gemini"), "Hello, Gemini! This is an Atomic greeting.");
    }
}
