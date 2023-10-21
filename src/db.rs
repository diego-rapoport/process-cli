use rusqlite::{params, Connection, Error, ToSql};

use crate::process::{Process, Step};

#[derive(Debug)]
pub struct Db(Connection);

impl Db {
    pub fn open() -> Result<Db, Error> {
        let conn = Connection::open("process.sqlite")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS processes (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                num_steps INTEGER NOT NULL,
            );
        ",
            [],
        )?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS steps (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                step_num INTEGER NOT NULL,
                description TEXT,
                process_id INTEGER,
                FOREIGN KEY(process_id) REFERENCES processes(id)
                    ON UPDATE CASCADE
                    ON DELETE SET NULL
            );
        ",
            [],
        )?;

        Ok(Db(conn))
    }

    pub fn save_process(&self, process: &Process) -> Result<(), Error> {
        self.0.execute(
            "INSERT OR REPLACE INTO processes (id, name, num_steps) VALUES (?1, ?2, ?3)",
            params![process.id, process.name, process.num_steps],
        )?;

        for step in process.steps.clone().into_iter() {
            self.save_step(&step, self.0.last_insert_rowid().try_into().unwrap());
        }

        Ok(())
    }

    pub fn save_step(&self, step: &Step, process_id: usize) -> Result<(), Error> {
        self.0.execute("INSERT OR REPLACE INTO steps (id, name, step_num, description, process_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![step.id, step.name, step.step_num, step.description, process_id])?;
        Ok(())
    }

    pub fn get_steps_from_process(&self, id: usize) -> Result<Vec<Step>, Error> {
        let mut steps = self.0.prepare("SELECT id, name, step_num, description, process_id FROM steps")?;
        let mut vec_steps: Vec<Step> = vec![];
        steps.query_map([], |row| {
            Ok(vec_steps.push(Step {
                id: row.get(0)?,
                name: row.get(1)?,
                step_num: row.get(2)?,
                description: row.get(3)?,
            }))
        })?;

        Ok(vec_steps)
    }

    pub fn get_all_processes(&self) -> Result<Option<Process>, Error> {
        let mut processes = self.0.prepare("SELECT id, name, num_steps, steps FROM processes")?;

        let mut process_iter = processes.query_map([], |row| {
            let id = row.get(0)?;
            let name: String = row.get(1)?;
            let num_steps: usize = row.get(2)?;
            let steps = self.get_steps_from_process(id).unwrap();
            Ok(Process {
                id: Some(id),
                name,
                num_steps,
                steps,
            })
        })?;

        if let Some(process) = process_iter.next() {
            return Ok(Some(process?));
        } else {Ok(None)}
    }
}
