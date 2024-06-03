import json

res = []

for i in range(45, 201):
    res.append(
    {
        "id": i,
        "date": "2022",
        "link": "",
        "title": "",
        "tags": ["MC原味生存", "MC地下城", "MC盖地图", "MC空岛生存"],
        "hours": ""
    },
)

with open('./src/assets/numbered_vod.json', 'w', encoding='utf-8') as f:
    # 移動檔案指針到開頭，並寫入更新後的行到文本檔案中
    json.dump(res, f, ensure_ascii=False)

print("已將 JSON 檔案中的每一行設置為文本檔案對應行的值。")