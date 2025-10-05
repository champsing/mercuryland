import sqlite3
import json

# Connect to the database
conn = sqlite3.connect('data/sqlite.db')
cursor = conn.cursor()

# Delete all items from penalty table
cursor.execute('DELETE FROM penalty')

# Load data from penalty_4.json
with open('web/assets/data/penalty_4.json', 'r', encoding='utf-8') as f:
    penalties = json.load(f)

# Insert each penalty
for penalty in penalties:
    date_val = penalty['date']
    name_val = penalty['name']
    detail_val = penalty.get('description', '')
    state_val = penalty['state']
    # Swap history to (state, date)
    history_swapped = [[entry[1], entry[0]] for entry in penalty['history']]
    history_val = json.dumps(history_swapped, ensure_ascii=False)
    
    cursor.execute('''
        INSERT INTO penalty (date, name, detail, state, history)
        VALUES (?, ?, ?, ?, ?)
    ''', (date_val, name_val, detail_val, state_val, history_val))

# Commit and close
conn.commit()
conn.close()

print("Data inserted successfully.")