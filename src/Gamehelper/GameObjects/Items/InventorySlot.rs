use std::net::Incoming;
use crate::gameobjects::item_handler::Equipmintslots;
use crate::gameobjects::item_handler::Item;

#[derive(Clone)]
pub(crate) struct Inventoryslot{
    name: String,
    eq_slot:  Equipmintslots,
}

 impl Inventoryslot {

    pub fn empty()-> Self {
       Self {
            name: "Empty".into(),
            eq_slot: Equipmintslots::None,
        }
    }
}



impl Item for Inventoryslot {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> Vec<crate::gameobjects::item_handler::Equipmintslots> {

        vec![self.eq_slot.clone()]
    }

    fn get_des(&self) -> &str {
        "Empty"

    }


}
