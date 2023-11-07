use std::io;

use crate::step::Step;

pub fn generate_cli_steps(steps: usize) -> Vec<Step> {
    let mut array_steps: Vec<Step> = vec![];
    for n in 1..=steps {
        println!("Please provide the name of the step number {} -> ", n);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let step_name = name.trim_end().to_string();
        println!("Now provide a description -> ");
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");
        let step_description = description.trim_end().to_string();
        let new_step: Step = Step {
            id: None,
            name: step_name,
            step_num: n,
            description: step_description,
            is_done: false,
        };
        array_steps.push(new_step);
    }

    array_steps
}
