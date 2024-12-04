use crate::gameobjects::encounter::Encounter;

#[derive(Clone)]
pub(crate) struct Trap {

}

impl Encounter for Trap {
    fn get_Name(&self) ->&str {
        todo!()
    }

    fn get_Type(&self) -> &str {
        todo!()
    }

    fn get_description(&self) -> &str {
        todo!()
    }
}