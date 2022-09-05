use super::error_handling::error_parser;
use crate::DB_CONNECTION;
use rusqlite::{Connection, Result};

#[tauri::command]
pub fn check_worker_id(state: tauri::State<DB_CONNECTION>, worker_id: &str) -> String {
    if worker_id.is_empty() {
        return format!("Error: Input can not be empty!");
    } else {
        let id: String = worker_id.chars().filter(|c| !c.is_whitespace()).collect();
        let conn: Connection = Connection::open(&state.inner().0).expect("some database error");

        let res: Result<String> = conn.query_row(
            "SELECT * FROM worker_pool WHERE worker_id=?1 ",
            &[&id],
            |row| row.get(1),
        );
        error_parser(&res)
    }
}

#[tauri::command]
pub fn fetch_table(state: tauri::State<DB_CONNECTION>) -> Vec<(String, String)> {
    let conn = Connection::open(&state.inner().0).expect("some Database error");
    let mut stmt = conn.prepare("SELECT * FROM worker_pool").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut content = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let row_tuple: (String, String) = (row.get(0).unwrap(), row.get(1).unwrap());
        content.push(row_tuple);
    }
    content
}
