CREATE TABLE IF NOT EXISTS abilities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL
);
