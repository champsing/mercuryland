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

# %%
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
                
                assert check_name(entry["name"], m241118[entry["id"]][1])
                i = entry["id"]
                entry["id"] = m241118[i][0]
                entry["name"] = m241118[i][1]
            except KeyError:
                print(f"KeyError: [{commit}] {entry['id']}, {entry['name']}")

for commit, file in files1:
    print(commit, file)

# %%
