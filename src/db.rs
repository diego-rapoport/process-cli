use rusqlite::{params, Connection, Error, ToSql, config::DbConfig};

use crate::step::Step;
use crate::process::Process;
use crate::parsed::ParsedInfo;

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
                name TEXT NOT NULL UNIQUE,
                num_steps INTEGER NOT NULL
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
        let mut process = self
            .0
            .query_row("SELECT id, name, num_steps, is_finished FROM processes WHERE id = :id", params![id], |row| { 
                let id: usize = row.get(0)?;
                let name: String = row.get(1)?;
                let num_steps: usize = row.get(2)?;
                let steps = self.get_steps_from_process(id)?;
                let is_finished: bool = row.get(3)?;
                Ok(Process {
                    id: Some(id),
                    name,
                    num_steps,
                    steps,
                    is_finished,
                })
            }
            )?;

        Ok(process)
    }

    pub fn get_all(&self) -> Result<Vec<ParsedInfo>, Option<Error>> {
        let mut stmt = self.0.prepare("
            SELECT processes.id as process_id, processes.name as process_name, processes.num_steps as process_num_steps, processes.is_finished as process_finished,
            steps.id as step_id, steps.name as step_name, steps.step_num as step_num, steps.description as step_description, steps.is_done as step_done
            FROM processes INNER JOIN steps ON processes.id = steps.process_id")?;

        let mut info_iter = stmt.query_map([], |row| {
            let process_id = row.get(0)?;
            let process_name = row.get(1)?;
            let process_num_steps = row.get(2)?;
            let process_finished = row.get(3)?;
            let step_id = row.get(4)?;
            let step_name = row.get(5)?;
            let step_num = row.get(6)?;
            let step_description = row.get(7)?;
            let step_done = row.get(8)?;
            Ok(ParsedInfo {
                process_id,
                process_name,
                process_num_steps,
                process_finished,
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
}
