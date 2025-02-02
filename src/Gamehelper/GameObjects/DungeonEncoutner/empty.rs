use crate::gameobjects::encounter::Encounter;
#[derive(Clone)]
pub(crate) struct Empty {
    name: String,
    des: String,
}

impl Empty {
    pub fn new(name: String, des: String) -> Empty {
        Self { name, des }
    }
}

impl Encounter for Empty {
    fn get_Name(&self) -> &str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &"Empty"
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}
