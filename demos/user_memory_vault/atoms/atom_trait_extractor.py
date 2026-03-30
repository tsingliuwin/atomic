# demos/user_memory_vault/atoms/atom_trait_extractor.py
import sys
import json
import re

### @atomic
### id: atom_trait_extractor
### type: LOGIC
### language: python
### runtime: python3
### description: Extracts interests and personality traits from normalized user data.
### inputs: ["JSON:normalized_data"]
### outputs: ["JSON:user_traits"]
### @atomic

def extract_traits(data: dict):
    # 模拟简单的特质提取逻辑
    content = data.get("raw_content", str(data)).lower()
    traits = {
        "interests": [],
        "sentiment": "neutral",
        "tags": []
    }
    
    # 关键字匹配
    keywords = {
        "tech": ["rust", "python", "code", "ai", "atomic"],
        "lifestyle": ["travel", "food", "music"],
        "finance": ["stock", "market", "money"]
    }
    
    for category, words in keywords.items():
        if any(word in content for word in words):
            traits["interests"].append(category)
            
    # 情感分析（极简版）
    positive_words = ["love", "great", "evolving", "fast", "powerful"]
    if any(word in content for word in positive_words):
        traits["sentiment"] = "positive"
        
    return traits

if __name__ == "__main__":
    try:
        input_data = json.loads(sys.argv[1])
        result = extract_traits(input_data)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)
