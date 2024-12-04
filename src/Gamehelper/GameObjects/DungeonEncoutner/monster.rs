use crate::gameobjects::encounter::Encounter;


#[derive(Clone)]
pub(crate) struct Monster {
    name: String,
    m_type: String,
    des: String
}

impl Monster {
    pub fn new(name: String) -> Self {
        Self {
            name,
            m_type: "Monster".into(),
            des: "A monster".into(),
        }
    }
}



impl Encounter for Monster{
    fn get_Name(&self) -> &str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.m_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}