#[derive(Clone, Debug)]
pub struct Step {
    pub id: Option<usize>,
    pub name: String,
    pub step_num: usize,
    pub description: String,
    pub is_done: bool,
}
