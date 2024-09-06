const { existsSync, writeFileSync } = require("fs");
const sqlite = require("sqlite3").verbose();

if (!existsSync("backend/data.db")) {
    writeFileSync("backend/data.db", "");
    const db = new sqlite.Database("backend/data.db");
    db.exec(
        "CREATE TABLE IF NOT EXISTS files (thread_id TEXT NOT NULL, file_name TEXT NOT NULL);",
    );
}
