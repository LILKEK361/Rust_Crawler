use crate::gameobjects::encounter::Encounter;

#[derive(Clone)]
pub(crate) struct Trap {
    name: String,
    t_type: String,
    des: String,
    dmg: i8,
    skillcheck: i8,
    spotted: bool,
    disarmed: bool,

}

impl Trap {
    pub(crate) fn new() -> Trap {
       Self {
            name: "Empty".to_string(),
            t_type: "Empty".into(),
            des: "Somthings different to the other rooms.\nYou can't quite say what it is.\nBut there is something\nMaybe just you paranoire or maybe something else".into(),
            skillcheck: 10,
            dmg: 1,
            spotted: false,
            disarmed: false,
       }
    }
}

impl Trap {
    pub fn get_dmg(&self) -> &i8 {
            &self.dmg
    }

    pub fn make_skillcheck(&self, skillmod: i8 ) -> bool {
        if(10 + skillmod > self.skillcheck){
            true
        }else {
            false
        }
    }

    pub fn is_spotted(&mut self){
        self.name = "Trap".into();
        self.des = "Now you can see it.\nThe floor tiles are different.\nYou spott small spikes embeded into the ground.".into();
        self.t_type = "Trap".into();
        self.spotted = true;
    }
}

impl Encounter for Trap {
    fn get_Name(&self) ->&str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.t_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}