# demos/python_rust_bridge/atoms/atom_metadata_extractor.py
import re
import json
import sys
import hashlib

### @atomic
### id: atom_metadata_extractor
### type: UTILITY
### language: python
### runtime: python3
### description: Extracts AMS metadata from source code comments (Doc-as-Contract).
### inputs: ["String:file_path"]
### outputs: ["JSON:metadata"]
### @atomic

def extract_metadata(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 提取 @atomic 块
    pattern = r'@atomic\n(.*?)\n.*?@atomic'
    match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        return None
    
    lines = match.group(1).split('\n')
    metadata = {}
    for line in lines:
        line = line.replace('###', '').replace('/*', '').replace('*/', '').strip()
        if ':' in line:
            key, value = line.split(':', 1)
            key = key.strip()
            value = value.strip()
            # 处理数组格式
            if value.startswith('[') and value.endswith(']'):
                metadata[key] = json.loads(value)
            else:
                metadata[key] = value
    
    # 注入物理路径
    metadata['path'] = file_path
    return metadata

if __name__ == "__main__":
    target_file = sys.argv[1] if len(sys.argv) > 1 else ""
    if target_file:
        result = extract_metadata(target_file)
        print(json.dumps(result, indent=2))
