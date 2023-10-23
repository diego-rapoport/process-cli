#[derive(Clone, Debug)]
pub struct Step {
    pub id: Option<usize>,
    pub name: String,
    pub step_num: usize,
    pub description: String,
}

#[derive(Debug)]
pub struct Process {
    pub id: Option<usize>,
    pub name: String,
    pub num_steps: usize,
    pub steps: Vec<Step>,
}

impl Process {
    fn check_numstep_exist(&self, new_step: &Step) -> bool {
        for step in &self.steps {
            if step.step_num == new_step.step_num {
                return true;
            }
        }
        false
    }

    fn check_numsteps(&self) -> bool {
        if &self.num_steps != &self.steps.len() {
            return false;
        }
        true
    }

    fn add_step(&mut self, new_step: Step) {
        self.steps.push(new_step)
    }

    fn swap_step(&mut self, new_step: &mut Step) {
        let first: usize = self.steps[new_step.step_num].step_num;
        self.steps.swap(first, new_step.step_num)
    }

    fn update_step(&mut self, new_step: &mut Step) {
        if self.check_numstep_exist(new_step) {
            let _ = &self.swap_step(new_step);
        }
        self.add_step(new_step.to_owned());
    }
}
