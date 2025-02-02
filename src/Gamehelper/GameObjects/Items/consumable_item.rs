use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};
#[derive(Clone)]
pub struct Consumable {
    name: String,
    des: String,
    value: u8,
    uses: u8,
    rarity: Raritys,
    buf: u8,
}

impl Consumable {
    pub fn new(name: String, des: String, rarity: Raritys, uses: u8, buf: u8, value: u8) -> Self {
        Self {
            name,
            des,
            rarity,
            uses,
            buf,
            value,
        }
    }

    pub fn get_buf(&self) -> &u8 {
        &self.buf
    }

    pub fn used(&mut self) {
        self.uses = self.uses - 1;
    }
    pub fn get_uses(&self) -> &u8 {
        &self.uses
    }
}

impl Item for Consumable {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> &Equipmintslots {
        &Equipmintslots::None
    }

    fn get_des(&self) -> &str {
        &self.des
    }

    fn get_rarity(&self) -> &Raritys {
        &self.rarity
    }

    fn get_value(&self) -> &u8 {
        &self.value
    }

    fn get_bonus_dmg(&self) -> &u8 {
        &0
    }
}
