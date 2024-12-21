use crate::gameobjects::encounter::Encounter;

#[derive(Clone)]
pub(crate) struct Trap {
    name: String,
    t_type: String,
    des: String
}

impl Trap {
    pub(crate) fn new() -> Trap {
       Self {
            name: "Trap".to_string(),
            t_type: "Trap".into(),
            des: "Seems like a trap to me".into(),
       }
    }
}

impl Encounter for Trap {
    fn get_Name(&self) ->&str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.t_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}