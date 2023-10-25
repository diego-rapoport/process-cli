#[derive(Debug)]
pub struct ParsedInfo {
    pub process_id: usize,
    pub process_name: String,
    pub process_num_steps: usize,
    pub process_finished: bool,
    pub step_id: usize,
    pub step_name: String,
    pub step_num: usize,
    pub step_description: String,
    pub step_done: bool,
}

