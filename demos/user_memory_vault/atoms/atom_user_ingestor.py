# demos/user_memory_vault/atoms/atom_user_ingestor.py
import sys
import json

### @atomic
### id: atom_user_ingestor
### type: IO
### language: python
### runtime: python3
### description: Normalizes raw text or JSON input into a standardized data stream for analysis.
### inputs: ["String:raw_input"]
### outputs: ["JSON:normalized_data"]
### @atomic

def ingest(raw_input: str):
    # 模拟归一化逻辑：去除噪音并结构化
    # 如果是 JSON 字符串，则解析；否则包装为原始数据对象。
    try:
        data = json.loads(raw_input)
    except:
        data = {"raw_content": raw_input, "source": "unstructured_text"}
    return data

if __name__ == "__main__":
    input_str = sys.argv[1] if len(sys.argv) > 1 else ""
    result = ingest(input_str)
    print(json.dumps(result))
