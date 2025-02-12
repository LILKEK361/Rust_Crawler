use crate::gameobjects::encounter::Encounter;

#[derive(Clone)]
pub(crate) struct Trap {
    name: String,
    t_type: String,
    des: String,
    dmg: u8,
    skillcheck: i8,
    spotted: bool,
    disarmed: bool,
}

impl Trap {
    pub(crate) fn new(name: String, des: String, dmg: u8) -> Trap {
        Self {
            name,
            t_type: "Empty".into(),
            des,
            skillcheck: 10,
            dmg,
            spotted: false,
            disarmed: false,
       }
    }
}

impl Trap {
    pub fn get_dmg(&self) -> &u8 {
        &self.dmg
    }

    pub fn make_skillcheck(&self, skillmod: i8) -> bool {
        if (10 + skillmod > self.skillcheck) {
            true
        } else {
            false
        }
    }

    pub fn is_spotted(&mut self) {
        self.name = "Trap".into();
        self.des = "Now you can see it.\nThe floor tiles are different.\nYou spott small spikes embeded into the ground.".into();
        self.t_type = "Trap".into();
        self.spotted = true;
    }
}

impl Encounter for Trap {
    fn get_Name(&self) -> &str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.t_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}
