use std::net::Incoming;
use crate::gameobjects::item_handler::Equipmintslots;

pub(crate) struct Inventoryslot<'a>{
    name: &'a str,
    eq_slot: &'a [Equipmintslots]
}

 impl Inventoryslot<'static> {
    pub fn empty()-> Self {
        Self {
            name: &"Empty",
            eq_slot: &[Equipmintslots::None],
        }
    }
}

impl crate::gameobjects::item_handler::Item for Inventoryslot<'static> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> &[crate::gameobjects::item_handler::Equipmintslots] {

        &self.eq_slot
    }

    fn get_des(&self) -> &str {
        "Empty"

    }


}
