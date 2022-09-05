use rusqlite::Error;

pub trait Message {
    fn send_message(&self) -> String;
}

impl Message for usize {
    fn send_message(&self) -> String {
        let zero: usize = 0;
        if self == &zero {
            format!("Error: {} rows affected", self)
        } else {
            format!("{} rows affected", self)
        }
    }
}
impl Message for String {
    fn send_message(&self) -> String {
        format!("{}", self)
    }
}

pub fn error_parser<T: Message>(res: &Result<T, Error>) -> String {
    match res {
        Ok(message) => message.send_message(),
        Err(err) => format!("Error: {}", err),
    }
}
