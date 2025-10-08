# %% Get paths
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
OUTPUT = Path("web/assets/data/penalty_1.json")

# %% Get commits
import subprocess
handle = subprocess.run(["git", "log", "--follow", "--pretty=format:%H %cs", "--", "web/assets/data/penalty.json"], capture_output=True, text=True)

commits: list[tuple[str, str]] = []
for line in handle.stdout.strip().splitlines():
    if not line.strip():
        continue
    try:
        commit_hash, commit_date = line.strip().split()
        commits.append((commit_hash, commit_date))
    except ValueError:
        print(f"Skipping malformed line: {line!r}")
commits.reverse()  # oldest -> newest
print(commits)

# %% Get file changes
import json
files0: list[tuple[str, list[dict]]] = []
for i in range(0, len(commits)):
    if i < 15:
        target = "src/assets/penalty.json"
        handle = subprocess.run(["git", "show", f"{commits[i][0]}:{target}"], capture_output=True, text=True)
        print(commits[i], json.loads(handle.stdout))
        files0.append((commits[i], json.loads(handle.stdout)))
    if 15 <= i < 46:
        target = "src/assets/data/penalty.json"
        handle = subprocess.run(["git", "show", f"{commits[i][0]}:{target}"], capture_output=True, text=True)
        print(commits[i], json.loads(handle.stdout))
        files0.append((commits[i], json.loads(handle.stdout)))
    if 46 <= i:
        target = "web/assets/data/penalty.json"
        handle = subprocess.run(["git", "show", f"{commits[i][0]}:{target}"], capture_output=True, text=True)
        print(commits[i], json.loads(handle.stdout))
        files0.append((commits[i], json.loads(handle.stdout)))

# %% Normalization
import re
def normalize(s: str) -> str:
    s = re.sub(r'\s+', '', s)
    emoji_pattern = re.compile(
        "["
        "\U0001F600-\U0001F64F"  # Emoticons
        "\U0001F300-\U0001F5FF"  # Misc symbols and pictographs
        "\U0001F680-\U0001F6FF"  # Transport and map symbols
        "\U0001F700-\U0001F77F"  # Alchemical symbols
        "\U0001F780-\U0001F7FF"  # Geometric shapes extended
        "\U0001F800-\U0001F8FF"  # Supplemental arrows-C
        "\U0001F900-\U0001F9FF"  # Supplemental symbols and pictographs
        "\U0001FA00-\U0001FA6F"  # Chess symbols etc.
        "\U0001FA70-\U0001FAFF"  # Symbols and pictographs extended-A
        "\U00002700-\U000027BF"  # Dingbats
        "\U00002600-\U000026FF"  # Misc symbols (e.g., sun, umbrella)
        "]+",
        flags=re.UNICODE
    )
    s = emoji_pattern.sub('', s)
    return s

# %% Mapping for id changes on 2024-11-18
import copy
commit0 = next((x for x in files0 if x[0][0] == "987ff4686d1d4d1658de1bf6ab7ca9e251cd1a95"))
commit1 = next((x for x in files0 if x[0][0] == "5c1e4f7e20328f18fcde218024319959eae3aebd"))

assert len(commit0[1]) == len(commit1[1])

m241118 = {}
for (old_entry, new_entry) in zip(commit0[1], commit1[1]):
    assert old_entry["name"] == new_entry["name"]
    m241118[old_entry["id"]] = (new_entry["id"], new_entry["name"])

# Manual fix for SkyBlock 100等
m241118[96] = m241118[97]

print(m241118)

del commit0
del commit1

files1 = copy.deepcopy(files0)
for commit, file in files1:
    if commit[1] < "2024-11-18" or commit[0] == "987ff4686d1d4d1658de1bf6ab7ca9e251cd1a95":
        for entry in file:
            try:
                def check_name(new_name: str, old_name: str) -> str:
                    new_name = normalize(new_name)
                    old_name = normalize(old_name)
                    if new_name == old_name:
                        return True
                    elif new_name == "玩slimemo" and old_name == "玩smilemo":
                        return True
                    elif new_name == "打完星鐵主線(️雅利洛-VI)" and old_name == "打完星鐵主線":
                        return True
                    elif new_name == "直播遊玩黎明死線倖存者或殺手，直到熒虹徽章20個(️已完成**4**/20個)" and old_name == "直播遊玩黎明死線倖存者或殺手，直到熒虹徽章20個":
                        return True
                    elif new_name == "通關冰與火之歌前七關卡" and old_name == "通關冰與火之舞前七關卡":
                        return True
                    else:
                        print(f"Normalizing mismatch: {new_name} != {old_name}")
                        return False
                
                i = entry["id"]
                assert check_name(entry["name"], m241118[i][1])
                entry["id"] = m241118[i][0]
                entry["name"] = m241118[i][1]
            except KeyError:
                print(f"KeyError: [{commit}] {entry['id']}, {entry['name']}")

for commit, file in files1:
    print(commit, file)

del files0
del m241118

# %% Mapping for id changes on 2024-11-20
commit0 = next((x for x in files1 if x[0][0] == "6c79c029cd4e39371b9c60511ef486abb415c911"))
commit1 = next((x for x in files1 if x[0][0] == "68224d1b79ae037c2974e9a94ef4a54fced6e6ec"))
assert len(commit0[1]) + 1 == len(commit1[1])

t0 = {}
for entry in commit0[1]:
    t0[entry["name"]] = (entry["id"], 0)
for entry in commit1[1]:
    if entry["name"] in t0:
        t0[entry["name"]] = (t0[entry["name"]][0], entry["id"])
    else:
        print(f"New entry: {entry['name']}")
        t0[entry["name"]] = (0, entry["id"])

m241120 = {}
for name, (old_id, new_id) in t0.items():
    m241120[old_id] = (new_id, name)

# Manual fix for 加班台2小時 and 開加班台
m241120[17] = (23423, "加班台2小時")
m241120[18] = (23423, "開加班台")
m241120[50] = (23910.6, "開加班台")
m241120[57] = (23924.2, "加班台2小時")

print(m241120)

del commit0
del commit1
del t0

files2 = copy.deepcopy(files1)
for commit, file in files2:
    if commit[1] < "2024-11-20" or commit[0] in [
        "d8d563c13e77b51dcf2e260a6605c584e30f2eb6", 
        "7a8475fbc319b051dd4d04cee8c7f4ae9f6bb60e",
        "6c79c029cd4e39371b9c60511ef486abb415c911",
    ]:
        for entry in file:
            try:
                def check_name(new_name: str, old_name: str) -> str:
                    new_name = normalize(new_name)
                    old_name = normalize(old_name)
                    if new_name == old_name:
                        return True
                    else:
                        print(f"Normalizing mismatch: {new_name} != {old_name}")
                        return False
                
                i = entry["id"]
                assert check_name(entry["name"], m241120[i][1])
                entry["id"] = m241120[i][0]
                entry["name"] = m241120[i][1]
            except KeyError:
                print(f"KeyError: [{commit}] {entry['id']}, {entry['name']}")

for commit, file in files2:
    print(commit, file)

del files1
del m241120

# %% Manual fix for 開每日迷因頻道 日更連續至少1星期
files3 = copy.deepcopy(files2)
for commit, file in files3:
    if commit[0] == "1da5ad196b6d807715e7eacce4334a3e230720c1":
        for entry in file:
            if entry["id"] == 250202.4:
                assert entry["name"] == "開每日迷因頻道 日更連續至少1星期"
                entry["id"] = 250202.5

del files2

# %%
commit0 = next((x for x in files3 if x[0][0] == "39f4a0530ea7a62e9b7fb649703a155eaf967090"))
commit1 = next((x for x in files3 if x[0][0] == "e7fa087599e25323209f64f9d33101f1de946275"))

assert len(commit0[1]) + 4 == len(commit1[1])

m250608 = {}
for (old_entry, new_entry) in zip(commit0[1], commit1[1]):
    assert old_entry["name"] == new_entry["name"]
    m250608[old_entry["id"]] = (new_entry["id"], new_entry["name"])

# After 0324, 未生效 are randomly assigned new ids
m250608[67012669] = (162, "玩狂亂之境4 打敗BOSS百變獸")
m250608[9026498] = (165, "考試考歐洲所有國家首都 不算私人國家而已 三星期後考")
m250608[8772780] = (166, "玩Mad Max")

files4 = copy.deepcopy(files3)
for commit, file in files4:
    if commit[1] < "2025-06-08":
        for entry in file:
            try:
                def check_name(new_name: str, old_name: str) -> str:
                    new_name = normalize(new_name)
                    old_name = normalize(old_name)
                    if new_name == old_name:
                        return True
                    elif new_name == "加班台2小時" and old_name == "開加班台":
                        return True
                    elif new_name == "Project2M16目標全達成" and old_name == "Project2M只要把你現在找到的羊毛放到要放的位置就能用旁觀+創造帶大家看一下地圖和剩餘的羊毛放到該放的位置上":
                        return True
                    else:
                        print(f"[{commit}]Normalizing mismatch: {new_name} != {old_name}")
                        return False
                
                i = entry["id"]
                if i == 250309 and entry["name"] == "玩狂亂之境4 打敗BOSS百變獸":
                    print("Manual fix for 玩狂亂之境4打敗BOSS百變獸")
                    entry["id"] = 162
                elif i == 250511:
                    continue  # invalid id
                else:
                    assert check_name(entry["name"], m250608[i][1])
                    entry["id"] = m250608[i][0]
                    entry["name"] = m250608[i][1]
            except KeyError:
                print(f"KeyError: [{commit}] {entry['id']}, {entry['name']}")

for commit, file in files4:
    print(commit, file)

del m250608
del files3
