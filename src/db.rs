use rusqlite::{config::DbConfig, params, Connection, Error, ToSql};

use crate::parsed::ParsedInfo;
use crate::process::Process;
use crate::step::Step;

#[derive(Debug)]
pub struct Db(Connection);

impl Db {
    pub fn open() -> Result<Db, Error> {
        let conn = Connection::open("process.sqlite")?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true);

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS processes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                num_steps INTEGER NOT NULL,
                is_done BOOLEAN DEFAULT 0
            );
        ",
            [],
        )?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS steps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                step_num INTEGER NOT NULL,
                description TEXT,
                process_id INTEGER,
                is_done BOOLEAN DEFAULT 0,
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
            "INSERT INTO processes (name, num_steps) VALUES (?1, ?2)",
            params![process.name, process.num_steps],
        )?;

        let process_id = self.0.last_insert_rowid().try_into().unwrap();
        println!("Process id got: {}", process_id);
        for step in process.steps.clone().into_iter() {
            self.save_step(&step, process_id);
        }

        Ok(())
    }

    pub fn save_step(&self, step: &Step, process_id: usize) -> Result<(), Error> {
        self.0.execute("INSERT INTO steps (name, step_num, description, process_id) VALUES (?1, ?2, ?3, ?4)",
            params![step.name, step.step_num, step.description, process_id])?;
        Ok(())
    }

    pub fn get_steps_from_process(&self, id: usize) -> Result<Vec<Step>, Error> {
        let mut stmt = self.0.prepare(
            "SELECT id, name, step_num, description, process_id, is_done FROM steps WHERE process_id = :id",
        )?;
        let steps_iter = stmt.query_map(&[(":id", &id.to_string())], |row| {
            Ok(Step {
                id: row.get(0)?,
                name: row.get(1)?,
                step_num: row.get(2)?,
                description: row.get(3)?,
                is_done: row.get(4)?,
            })
        })?;

        let mut vec_steps: Vec<Step> = vec![];
        for step in steps_iter {
            vec_steps.push(step?)
        }

        Ok(vec_steps)
    }

    pub fn get_all_steps(&self) -> Result<Vec<Step>, Error> {
        let mut stmt = self
            .0
            .prepare("SELECT id, name, step_num, description, process_id, is_done FROM steps")?;

        let mut vec_steps: Vec<Step> = vec![];
        stmt.query_map([], |row| {
            Ok(vec_steps.push(Step {
                id: row.get(0)?,
                name: row.get(1)?,
                step_num: row.get(2)?,
                description: row.get(3)?,
                is_done: row.get(4)?,
            }))
        })?;

        Ok(vec_steps)
    }

    pub fn get_processes_from_id(&self, id: usize) -> Result<Process, Error> {
        let mut process = self.0.query_row(
            "SELECT id, name, num_steps, is_done FROM processes WHERE id = ?1",
            params![id],
            |row| {
                let id: usize = row.get(0)?;
                let name: String = row.get(1)?;
                let num_steps: usize = row.get(2)?;
                let steps = self.get_steps_from_process(id)?;
                let is_done: bool = row.get(3)?;
                Ok(Process {
                    id: Some(id),
                    name,
                    num_steps,
                    steps,
                    is_done,
                })
            },
        )?;

        Ok(process)
    }

    pub fn get_all(&self) -> Result<Vec<ParsedInfo>, Option<Error>> {
        let mut stmt = self.0.prepare("
            SELECT processes.id as process_id, processes.name as process_name, processes.num_steps as process_num_steps, processes.is_done as process_done,
            steps.id as step_id, steps.name as step_name, steps.step_num as step_num, steps.description as step_description, steps.is_done as step_done
            FROM processes INNER JOIN steps ON processes.id = steps.process_id")?;

        let mut info_iter = stmt.query_map([], |row| {
            let process_id = row.get(0)?;
            let process_name = row.get(1)?;
            let process_num_steps = row.get(2)?;
            let process_done = row.get(3)?;
            let step_id = row.get(4)?;
            let step_name = row.get(5)?;
            let step_num = row.get(6)?;
            let step_description = row.get(7)?;
            let step_done = row.get(8)?;
            Ok(ParsedInfo {
                process_id,
                process_name,
                process_num_steps,
                process_done,
                step_id,
                step_name,
                step_num,
                step_description,
                step_done,
            })
        })?;

        let mut infos = vec![];
        for info in info_iter {
            infos.push(info?)
        }

        Ok(infos)
    }

    pub fn update_process_name(&self, id: usize, name: String) -> () {
        match self.0.execute(
            "UPDATE processes SET name = ?1 WHERE id = ?2",
            params![name, id],
        ) {
            Ok(updated) => println!("Process succesfully updated!"),
            Err(err) => println!("Update failed: {}", err),
        }
    }

    pub fn update_step_name(&self, id: usize, name: String) -> () {
        match self.0.execute(
            "UPDATE steps SET name = ?1 WHERE id = ?2",
            params![name, id],
        ) {
            Ok(updated) => println!("Step succesfully updated!"),
            Err(err) => println!("Update failed: {}", err),
        }
    }

    pub fn update_step_description(&self, id: usize, description: String) -> () {
        match self.0.execute(
            "UPDATE steps SET description = ?1 WHERE id = ?2",
            params![description, id],
        ) {
            Ok(updated) => println!("Step succesfully updated!"),
            Err(err) => println!("Update failed: {}", err),
        }
    }

    pub fn toggle_process_done_toggle(&self, id: usize) {
        match self.0.execute("UPDATE processes SET is_done = (CASE WHEN is_done = 0 THEN 1 ELSE 0 END) WHERE id = ?1;", params![id]) {
            Ok(updated) => println!("Process toggled successfully!"),
            Err(err) => println!("Toggle failed: {}", err),
        }
    }

    pub fn toggle_step_done_toggle(&self, id: usize) {
        match self.0.execute(
            "UPDATE steps SET is_done = (CASE WHEN is_done = 0 THEN 1 ELSE 0 END) WHERE id = ?1;",
            params![id],
        ) {
            Ok(updated) => println!("Step toggled successfully!"),
            Err(err) => println!("Toggle failed: {}", err),
        }
    }

    pub fn delete_process(&self, id: usize) {
        match self
            .0
            .execute("DELETE processes WHERE id = ?1", params![id])
        {
            Ok(deleted) => println!("Process deleted successfully!"),
            Err(err) => println!("Delete failed: {}", err),
        }
    }

    pub fn delete_step(&self, id: usize) {
        match self.0.execute("DELETE steps WHERE id = ?1", params![id]) {
            Ok(deleted) => println!("Step deleted successfully!"),
            Err(err) => println!("Delete failed: {}", err),
        }
    }
}
