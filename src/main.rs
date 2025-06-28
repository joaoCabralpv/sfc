mod screen;

use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 192;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
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

    // Limit to max ~60 fps update rate
    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut c = 0;
        for i in buffer.iter_mut() {
            *i = c%5*0x10FF5D; // write something more funny here!
            c+=1
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}