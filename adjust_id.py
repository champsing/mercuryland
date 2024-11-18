import json

with open('./id_wrong.json', 'r', encoding = 'UTF-8') as wrong:
    TBE = json.load(wrong)

for i in range(len(TBE)) :
    TBE[i]['id'] = 14 + i

with open('./id_right.json', 'w', encoding = 'UTF-8') as right:
    json.dump(TBE, right, ensure_ascii=False)