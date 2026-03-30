# demos/python_rust_bridge/atoms/atom_data_cleaner.py
import sys
import re

/// Atom ID: atom_data_cleaner
/// Type: UTILITY
/// Description: Cleans raw text by trimming, lowercasing, and removing special characters.
def clean(text: str) -> str:
    # 1. Trim and lowercase
    text = text.strip().lower()
    # 2. Remove non-alphanumeric characters (except spaces)
    text = re.sub(r'[^a-zA-Z0-9\s]', '', text)
    return text

if __name__ == "__main__":
    input_text = sys.argv[1] if len(sys.argv) > 1 else ""
    print(clean(input_text))

# Self-Testing logic
def test_cleaner():
    raw = "  Hello, Atomic World!!!  "
    expected = "hello atomic world"
    assert clean(raw) == expected
    print("Self-test passed!")
