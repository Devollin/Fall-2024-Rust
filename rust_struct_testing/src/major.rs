#[derive(Debug, PartialEq)]
pub enum Major {
    ComputerScience,
    ComputerEngineering,
    Undefined,
}

impl Major {
    pub fn classify(major: &str) -> Self {
        match major {
            "ComputerScience" | "CS" => {Major::ComputerScience},
            "ComputerEngineering" | "CE" => {Major::ComputerEngineering},
            _ => {Major::Undefined},
        }
    }
}