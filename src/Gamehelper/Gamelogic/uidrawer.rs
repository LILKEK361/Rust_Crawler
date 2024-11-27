use std::io;
use std::char;


pub struct Uidrawer {
    MAXWIDTH: u8, // Chars
    MAXHEIGHT: u8, //Chars

    Gameview: Rectangel,
    Playerview: Rectangel,
    Inputview: Rectangel, 

}

struct Rectangel {
    x: u8,
    y: u8,
    width: u8,
    height: u8
}


impl Uidrawer {
    pub fn move_cursor(x: u8, y: u8) {
        // ANSI-Sequenz, um den Cursor zu bewegen
        print!("\x1B[{};{}H", y, x);
    }

    pub fn draw_basic_ui(&self){
        
        match Self::draw_rectangel(&self.Gameview) {
            Ok(_) => {
                let mut buffer = String::new();

                let mut _w = io::stdin().read_line(&mut buffer).unwrap();
                
                },
            Err(e) => {println!("Error: {}", e)}
        };
        
        

       
        
    }

    pub fn draw_rectangel(rect: &Rectangel) -> io::Result<()> {
        for i in 0..rect.width {
            Self::draw_char('#', (rect.x + i, rect.y));
            Self::draw_char('#', (rect.x + i, rect.y + rect.height));
        }

        for i in 0..rect.height {
            Self::draw_char('#', (rect.x, rect.y + i));
            Self::draw_char('#', (rect.x + rect.width, rect.y + i));
        }

        Ok(())
    }

    pub fn drawline(width: u8, coords: (u8, u8)){
        Self::move_cursor(coords.0, coords.1);
        for  i in 0..width {
            print!("#")
        }
    }

    pub fn draw_char(char: char, coords: (u8, u8)){
        Self::move_cursor(coords.0, coords.1);
        print!("{}", char);
    }

    pub fn new(maxheigth: u8, maxwidth: u8) -> Self{
        Self {
            MAXHEIGHT: maxheigth,
            MAXWIDTH: maxwidth,
            Gameview: Rectangel {x: 0, y: 0, width: maxwidth - 40, height: maxheigth - 10},
            Playerview: Rectangel {x: maxwidth - 40, y:0 , width: 40, height: maxheigth - 10},
            Inputview: Rectangel {x: 0, y: maxheigth - 10, width: maxwidth, height: 10}

        
        } 
    }
}