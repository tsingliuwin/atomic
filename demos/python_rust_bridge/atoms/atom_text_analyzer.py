# demos/python_rust_bridge/atoms/atom_text_analyzer.py
import sys
import json

/// Atom ID: atom_text_analyzer
/// Type: LOGIC
/// Description: A simple text analysis atom written in Python.
def analyze(text: str):
    return {
        "length": len(text),
        "word_count": len(text.split())
    }

if __name__ == "__main__":
    # 符合 Socket/StdIO 协议，输出 JSON 字符串
    input_text = sys.argv[1] if len(sys.argv) > 1 else ""
    result = analyze(input_text)
    print(json.dumps(result))
