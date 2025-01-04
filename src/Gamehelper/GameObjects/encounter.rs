use crate::gameobjects::monster::Monster;

#[derive(Clone)]
pub(crate) enum EncounterTypes{
    Monster(crate::gameobjects::monster::Monster),
    Trap(crate::gameobjects::trap::Trap),
    Empty,
    None,
    Goal(Monster),
    Treasure(crate::gameobjects::treasure::Treasure)

}





impl Encounter for EncounterTypes {
    fn get_Name(&self) -> &str {
        match self {
            EncounterTypes::Monster(monster) => monster.get_Name(),
            EncounterTypes::Trap(trap) => trap.get_Name(),
            EncounterTypes::Empty => "Empty",
            EncounterTypes::None => "None",
            EncounterTypes::Goal(monster) => {
                if(monster.is_alive()){
                    &monster.name
                } else {
                    "Goal"
                }
            },
            EncounterTypes::Treasure(treasure) => treasure.get_Name(),
        }
    }

    fn get_Type(&self) -> &str {
        match self {
            EncounterTypes::Monster(monster) => "Monster",
            EncounterTypes::Trap(trap) => "Trap",
            EncounterTypes::Empty => "Empty",
            EncounterTypes::None => "None",
            EncounterTypes::Goal(monster) => "Goal",
            EncounterTypes::Treasure(treasure) => "Treasure",
        }
    }

    fn get_description(&self) -> &str {
        match self {
            EncounterTypes::Monster(monster) => monster.get_description(),
            EncounterTypes::Trap(trap) => trap.get_description(),
            EncounterTypes::Empty => "A Empty room or is it?",
            EncounterTypes::None => "None",
            EncounterTypes::Goal(monster) => "Goal",
            EncounterTypes::Treasure(treasure) => treasure.get_description(),
        }
    }
}

pub(crate) trait Encounter: Sync + Send + Clone {
    fn get_Name(&self) -> &str;
    fn get_Type(&self) -> &str;
    fn get_description(&self) -> &str;
}





