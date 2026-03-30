# demos/user_memory_vault/atoms/atom_persona_synthesizer.py
import sys
import json
import time

### @atomic
### id: atom_persona_synthesizer
### type: MODEL
### language: python
### runtime: python3
### description: Synthesizes final user persona and influence metrics based on ingested data and traits.
### inputs: ["JSON:user_data", "JSON:user_traits"]
### outputs: ["JSON:user_persona"]
### @atomic

def synthesize(data, traits):
    # 最终画像生成逻辑
    persona = {
        "uid": data.get("uid", "unknown"),
        "timestamp": int(time.time()),
        "core_dna": {
            "top_interests": traits.get("interests", []),
            "sentiment_score": 1.0 if traits.get("sentiment") == "positive" else 0.5
        },
        "influence_score": len(traits.get("interests", [])) * 10
    }
    return persona

if __name__ == "__main__":
    try:
        # 接收两个 JSON 对象作为参数
        data = json.loads(sys.argv[1])
        traits = json.loads(sys.argv[2])
        print(json.dumps(synthesize(data, traits)))
    except:
        print(json.dumps({"error": "Failed to synthesize persona"}))
