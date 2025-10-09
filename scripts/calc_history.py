# %% Load history data
import json
with open("../web/assets/data/history.json", "r") as f:
    history = json.load(f)

# %% Process history data
# id -> [event, date]
h: dict[int, list[str, str]] = {}

p = ["未生效", "未開始", "進行中", "勉強過", "已完成"]

for commit, file in history:
    commit_date = commit[1]
    if commit_date == "2025-03-17":
        continue  # Skip this commit due to 基于20的自由心证, 所有的惩罚均已完成

    for entry in file:
        entry_date = entry["date"]
        id = entry["id"]
        if "status" in entry:
            state = entry["status"]
        elif "done" in entry:
            if entry["done"] == 0:
                state = "未開始"
            elif entry["done"] == 1:
                state = "已完成"
            elif entry["done"] == 2:
                state = "勉強過"
            elif entry["done"] == 3:
                state = "進行中"
            else:
                raise ValueError(f"Unknown done value: {entry['done']}")
        else:
            raise ValueError("Entry must have either 'status' or 'done' field")
        
        if state not in p:
            raise ValueError(f"Unknown status: {state}")
        
        if id not in h:
            if commit_date < "2025-03-22":
                # No 未生效 before 2025-03-22
                if state == "未生效":
                    raise ValueError("Found '未生效' before 2025-03-22")
                elif state == "未開始":
                    h[id] = [[state, entry_date]]
                else:
                    h[id] = [["未開始", entry_date], [state, commit_date]]
            else:
                first_date = min(commit_date, entry_date)
                if state == "未生效":
                    h[id] = [[state, first_date]]
                elif state == "未開始":
                    h[id] = [["未生效", entry_date], [state, entry_date]]
                else:
                    h[id] = [["未生效", entry_date], ["未開始", entry_date], [state, commit_date]]
        else:
            if h[id][-1][0] != state:
                h[id].append([state, commit_date])

# %% Manual fix
del h[250511] # Remove this id as it is not a valid entry
h[25] = [['未開始', '2023-04-09'], ['勉強過', '2024-06-01']]
h[81] = [['未開始', '2023-11-19'], ['已完成', '2024-06-01']]

with open("../web/assets/data/calc_history.json", "w") as f:
    json.dump(h, f, ensure_ascii=False, indent=2)
# %% 
import json
with open("../web/assets/data/calc_history.json", "r") as f:
    h = json.load(f)

with open("../web/assets/data/history.json", "r") as f:
    latest = json.load(f)[-1][1]

for entry in latest:
    id = str(entry["id"])
    if id not in h:
        raise ValueError(f"ID {id} not found in calc_history.json")
    entry["date"] = h[id][0][1]
    entry["history"] = h[id]

with open("../web/assets/data/latest.json", "w") as f:
    json.dump(latest, f, ensure_ascii=False, indent=2)
# %%
