use super::error_handling::error_parser;
use crate::DB_CONNECTION;
use rusqlite::Connection;

#[tauri::command]
pub fn delete_worker(state: tauri::State<DB_CONNECTION>, worker_id: &str) -> String {
    if worker_id.is_empty() {
        return format!("Error: Input can not be empty!");
    } else {
        let id: String = worker_id.chars().filter(|c| !c.is_whitespace()).collect();
        let conn: Connection = Connection::open(&state.inner().0).expect("some database error");
        let res = conn.execute(
            "DELETE FROM worker_pool
        WHERE worker_id LIKE (?1);",
            &[&id],
        );
        error_parser(&res)
    }
}
