use crate::gameobjects::encounter::Encounter;


#[derive(Clone)]
pub(crate) struct Monster {
    name: String,
    m_type: String,
    des: String,
    alvie: bool,
    hp: u8,


}

impl Monster {
    pub fn new(name: String) -> Self {
        Self {
            name,
            m_type: "Monster".into(),
            des: "A monster".into(),
            alvie: true,
            hp: 100,
        }
    }
}

impl Monster {
    pub fn is_alive(&self) -> bool {
        self.alvie
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