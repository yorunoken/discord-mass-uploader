use crate::command_trait::Command;

use crate::commands::{Ping, Upload};

pub fn get_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![Box::new(Ping), Box::new(Upload)]
}
