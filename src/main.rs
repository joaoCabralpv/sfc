use crate::screen::VRAMSIZE;

mod screen;


fn main() {

    //let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut memory :Box<[u8]>= Box::new([0; 524288]);

    let mut window = screen::init_window();


    memory[0]=0x01;
    memory[1]=0x23;
    memory[2]=0x34;
    memory[3]=0x56;
    memory[4]=0x78;
    memory[5]=0x89;
    memory[6]=0xAB;
    memory[7]=0xCD;
    memory[8]=0xEF;

    for i in 0..VRAMSIZE {
        memory[i]=memory[i%9];
    }

    while window.is_open(){
            screen::display(&mut window, &memory);
        }


    }