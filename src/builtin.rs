#[derive(Debug, PartialEq)]
pub enum BuiltInCommand {
    Exit,
    Echo,
    Type,
    Default
}

impl BuiltInCommand {
    pub fn from_str(command: &str) -> Self {
        match command {
            "exit" => BuiltInCommand::Exit,
            "echo" => BuiltInCommand::Echo,
            "type" => BuiltInCommand::Type,
            _ => BuiltInCommand::Default,
        }
    }
}
