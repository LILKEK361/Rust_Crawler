#[derive(Clone)]
pub(crate) enum EncounterTypes{
    Monster(crate::gameobjects::monster::Monster),
    Trap(crate::gameobjects::trap::Trap),
    Empty,
}



impl Encounter for EncounterTypes {
    fn get_Name(&self) -> &str {
        match self {
            EncounterTypes::Monster(monster) => monster.get_Name(),
            EncounterTypes::Trap(trap) => trap.get_Name(),
            EncounterTypes::Empty => "Empty",
        }
    }

    fn get_Type(&self) -> &str {
        match self {
            EncounterTypes::Monster(monster) => "Monster",
            EncounterTypes::Trap(trap) => "Trap",
            EncounterTypes::Empty => "Empty",
        }
    }

    fn get_description(&self) -> &str {
        todo!()
    }
}

pub(crate) trait Encounter: Sync + Send + Clone {
    fn get_Name(&self) -> &str;
    fn get_Type(&self) -> &str;
    fn get_description(&self) -> &str;
}





