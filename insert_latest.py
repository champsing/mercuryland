import sqlite3
import json

# Penalty state constants
PENALTY_STATE_INACTIVE = 0
PENALTY_STATE_NOT_STARTED = 1
PENALTY_STATE_IN_PROGRESS = 2
PENALTY_STATE_BARELY_DONE = 3
PENALTY_STATE_COMPLETED = 4

def map_status(status_str):
    mapping = {
        "未生效": PENALTY_STATE_INACTIVE,
        "未開始": PENALTY_STATE_NOT_STARTED,
        "進行中": PENALTY_STATE_IN_PROGRESS,
        "勉強過": PENALTY_STATE_BARELY_DONE,
        "已完成": PENALTY_STATE_COMPLETED
    }
    return mapping[status_str]

# Load vod data
with open('web/assets/data/vod.json', 'r', encoding='utf-8') as f:
    vod_data = json.load(f)

vod_dict = {vod['id']: vod for vod in vod_data}

# Load latest data
with open('web/assets/data/latest.json', 'r', encoding='utf-8') as f:
    penalties = json.load(f)

# Connect to the database
conn = sqlite3.connect('data/sqlite.db')
cursor = conn.cursor()

# Delete all items from penalty table
cursor.execute('DELETE FROM penalty')

# Insert each penalty
for penalty in penalties:
    date_val = penalty['date']
    name_val = penalty['name']
    status_str = penalty['status']
    state_val = map_status(status_str)
    
    # Process history
    history_list = []
    for h in penalty.get('history', []):
        h_state = map_status(h[0])
        h_date = h[1]
        history_list.append([h_state, h_date])
    history_val = json.dumps(history_list, ensure_ascii=False)
    
    # Process description to detail HTML
    detail_val = ''
    if 'description' in penalty:
        for desc in penalty['description']:
            if desc['type'] == 'text':
                detail_val += f'<p>{desc["text"]}</p>'
            elif desc['type'] == 'vod':
                vod_id = desc['uri_num']
                if vod_id in vod_dict:
                    link = vod_dict[vod_id]['link']
                    url = f'https://www.youtube.com/watch?v={link}'
                    detail_val += f'<p><a href="{url}">VOD</a></p>'
    
    cursor.execute('''
        INSERT INTO penalty (date, name, detail, state, history)
        VALUES (?, ?, ?, ?, ?)
    ''', (date_val, name_val, detail_val, state_val, history_val))

# Commit and close
conn.commit()
conn.close()

print("Data inserted successfully.")