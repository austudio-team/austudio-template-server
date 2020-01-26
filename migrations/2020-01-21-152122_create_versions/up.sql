-- Your SQL goes here
create table if not exists versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    major_version INTEGER NOT NULL,
    minor_version INTEGER NOT NULL,
    build_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    branch_name TEXT NOT NULL,
    created_time DATETIME DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime')) NOT NULL
)
