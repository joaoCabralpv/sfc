use minifb::{ScaleMode, Window, WindowOptions};


const WIDTH: usize = 256;
const HEIGHT: usize = 192;

pub const NPIXELS: usize = WIDTH*HEIGHT;
pub const VRAMSIZE: usize = NPIXELS/2;

pub fn init_window() -> Window{
    let mut opt = WindowOptions::default();
    opt.resize=true;
    opt.scale_mode=ScaleMode::AspectRatioStretch;
    let mut window = Window::new(
        "SFC",
        WIDTH,
        HEIGHT,
        opt,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(30);
    window
}

const BLACK_RGB      :u32 = 0x000000;
const DARK_GRAY_RGB  :u32 = 0x3F3F3F;
const RED_RGB        :u32 = 0xFF0000;
const DARK_RED_RGB   :u32 = 0x7F0000;
const GREEN_RGB      :u32 = 0x00FF00;
const DARK_GREEN_RGB :u32 = 0x007F00;
const YELLOW_RGB     :u32 = 0xFFFF00;
const DARK_YELLOW_RGB:u32 = 0x7F7F00;
const BLUE_RGB       :u32 = 0x0000FF;
const DARK_BLUE_RGB  :u32 = 0x00007F;
const PURPLE_RGB     :u32 = 0xFF00FF;
const DARK_PURPLE_RGB:u32 = 0x7F007F;
const CYAN_RGB       :u32 = 0x00FFFF;
const DARK_CYAN_RGB  :u32 = 0x007F7F;
const WHITE_RGB      :u32 = 0xFFFFFF;
const LIGHT_GREY_RGB :u32 = 0x7F7F7F;

const BLACK_SFC      :u8 = 0;
const DARK_GRAY_SFC  :u8 = 1;
const RED_SFC        :u8 = 2;
const DARK_RED_SFC   :u8 = 3;
const GREEN_SFC      :u8 = 4;
const DARK_GREEN_SFC :u8 = 5;
const YELLOW_SFC     :u8 = 6;
const DARK_YELLOW_SFC:u8 = 7;
const BLUE_SFC       :u8 = 8;
const DARK_BLUE_SFC  :u8 = 9;
const PURPLE_SFC     :u8 = 10; //A
const DARK_PURPLE_SFC:u8 = 11; //B
const CYAN_SFC       :u8 = 12; //C
const DARK_CYAN_SFC  :u8 = 13; //D
const WHITE_SFC      :u8 = 14; //E
const LIGHT_GREY_SFC :u8 = 15; //F


fn pallet_to_rgb(color:u8) -> u32{ // Converts the sfc color palet to rgb
    match color{
        BLACK_SFC => BLACK_RGB,
        DARK_GRAY_SFC => DARK_GRAY_RGB,
        RED_SFC => RED_RGB,
        DARK_RED_SFC => DARK_RED_RGB,
        GREEN_SFC => GREEN_RGB,
        DARK_GREEN_SFC => DARK_GREEN_RGB,
        YELLOW_SFC => YELLOW_RGB,
        DARK_YELLOW_SFC => DARK_YELLOW_RGB,
        BLUE_SFC => BLUE_RGB,
        DARK_BLUE_SFC => DARK_BLUE_RGB,
        PURPLE_SFC => PURPLE_RGB,
        DARK_PURPLE_SFC => DARK_PURPLE_RGB,
        CYAN_SFC => CYAN_RGB,
        DARK_CYAN_SFC => DARK_CYAN_RGB,
        WHITE_SFC => WHITE_RGB,
        LIGHT_GREY_SFC => LIGHT_GREY_RGB,
        _ => panic!("Invalid color")
    }
}

pub fn display(window: &mut Window, memory: &Box<[u8]>) {
    let mut buffer:Box<[u32]> = Box::new([0; NPIXELS]);
    for i in 0..VRAMSIZE {
        let byte:u8 = memory[i];
        let pixel1:u8=byte&0b00001111;
        let pixel2:u8=byte >> 4;
        buffer[i*2] = pallet_to_rgb(pixel1);
        buffer[1+(i*2)] = pallet_to_rgb(pixel2);
    }
    window
    .update_with_buffer(&buffer, WIDTH, HEIGHT)
    .unwrap();
}