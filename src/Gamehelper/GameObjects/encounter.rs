
pub(crate) enum EncounterTypes{
    Monster,
    Trap,
    Treasure,
    Boss,
    Empty
}


pub(crate) trait Encounter<> {
    fn get_Name(&self) -> &str;
    fn get_Type(&self) -> &str;
    fn get_description(&self) -> &str;

}

