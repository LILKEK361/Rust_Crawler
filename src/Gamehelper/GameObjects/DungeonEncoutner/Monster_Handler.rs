use crate::gameobjects::encounter::Encounter;

pub(crate) struct Monster {
    name: String,
}

impl Monster {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}


impl Encounter for Monster{
    fn get_Name(&self) -> &str {
        return &self.name
    }

    fn get_Type(&self) -> &str {
        todo!()
    }

    fn get_description(&self) ->&str {
        todo!()
    }
}