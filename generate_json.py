import json

res = []

for i in range(117, 201):
    res.append(
    {
        "id": i,
        "date": "2024",
        "link": "",
        "title": "【惡靈】",
        "tags": [],
        "duration": ""
    },
)

with open('./src/assets/numbered_vod.json', 'w', encoding='utf-8') as f:
    json.dump(res, f, ensure_ascii=False)

print("已將 JSON 檔案中的每一行設置為文本檔案對應行的值。")