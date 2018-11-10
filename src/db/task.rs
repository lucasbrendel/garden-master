use rusqlite::{Connection, Result, NO_PARAMS};

/// Standard task for things that need to be completed
#[derive(Debug)]
pub struct Task {
    /// Database unique id to identify instance
    pub id: i64,
    /// Task description of what needs to be done
    pub text: String,
}

impl Task {
    /// Create a new instance of a Task
    ///
    /// # Example
    ///
    /// ```
    /// let mgr = datamgr::DataMgr::new(String::from("./data/green-thumb-test-new_task.db"));
    /// let task = Task::new(&mgr.conn, String::from("Water garden"));
    /// assert_eq!(String::from("Water garden"), task.text);
    /// ```
    pub fn new(conn: &Connection, text: String) -> Self {
        conn.execute("INSERT INTO tasks (text) VALUES (?1)", &[&text])
            .unwrap();

        Task {
            id: conn.last_insert_rowid(),
            text,
        }
    }

    /// Access all tasks
    pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
        let mut tasks: Vec<Task> = Vec::new();
        let mut stmt = try!(conn.prepare("SELECT id, text FROM tasks"));
        let map_tasks = try!(stmt.query_map(NO_PARAMS, |row| Task {
            id: row.get(0),
            text: row.get(1),
        }));
        for task in map_tasks {
            info!("Accessing {:?}", task);
            tasks.push(task.unwrap());
        }
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::DataMgr;

    #[test]
    fn new_task() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-new_task.db"));
        let task = Task::new(&mgr.conn, String::from("Water garden"));
        assert_eq!(String::from("Water garden"), task.text);
    }

    #[test]
    fn get_tasks() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-new_task.db"));
        let task1 = Task::new(&mgr.conn, String::from("Water garden"));
        let task2 = Task::new(&mgr.conn, String::from("Weed garden"));
        let tasks = Task::get_tasks(&mgr.conn).unwrap();
        assert_eq!(task1.text, tasks[0].text);
        assert_eq!(task2.text, tasks[1].text);
    }
}
