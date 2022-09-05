use super::error_handling::error_parser;
use crate::DB_CONNECTION;
use rusqlite::Connection;

#[tauri::command]
pub fn create(state: tauri::State<DB_CONNECTION>, worker_id: &str, worker_name: &str) -> String {
    if worker_id.is_empty() || worker_name.is_empty() {
        return format!("Error: Input can not be empty!");
    } else {
        let id: String = worker_id.chars().filter(|c| !c.is_whitespace()).collect();
        let name: String = worker_name.chars().filter(|c| !c.is_whitespace()).collect();
        let conn: Connection = Connection::open(&state.inner().0).expect("some database error");

        let res = conn.execute(
            "INSERT INTO worker_pool (worker_id, worker_firstname) values (?1, ?2)",
            &[&id, &name],
        );
        error_parser(&res)
    }
}
