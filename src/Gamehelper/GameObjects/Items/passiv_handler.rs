#[derive(Clone)]
pub enum PassivTypes {
    HP(i8),
    AD(i8),
    None,
}

impl PassivTypes {
    pub fn create_passiv(passiv_string: String) -> PassivTypes {
        if (passiv_string.contains("HP")) {
            PassivTypes::HP(
                passiv_string.split(":").collect::<Vec<_>>()[0]
                    .parse::<isize>()
                    .unwrap() as i8,
            )
        } else if (passiv_string.contains("AD")) {
            PassivTypes::AD(
                passiv_string.split(":").collect::<Vec<_>>()[0]
                    .parse::<isize>()
                    .unwrap() as i8,
            )
        } else {
            PassivTypes::None
        }
    }

    pub fn to_string(&self) -> String {
        match (self) {
            PassivTypes::HP(v) => format!("+ {v} HP"),
            PassivTypes::AD(v) => format!("+ {v} AD"),
            _ => String::from("None"),
        }
    }
}
