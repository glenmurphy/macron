use winky::{Event, Key, Button};
use macron::{
    Macron,
    Cmd::{*}
};

#[tokio::main]
async fn main() {
    println!("Macro tool awaiting input");

    // Good for spamming sunshot
    let afk1 = Macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17100), Wait(50),  // -17100 for sunshot
        //MouseMove(0, -16500), Wait(50),

        Tap(Key::K),
        Wait(350),

        Tap(Key::Num2),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseMove(0, -500),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(750),
        
        MouseMove(-1200, 0), Wait(50),

        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(700),

        MouseMove(2200, 0), Wait(100),

        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(50),

        MouseMove(-1000, 0),

        Tap(Key::R),
        Tap(Key::W), Wait(100),
        Hold(Key::S, 450),
        Wait(1400),
    ], true);

    // Burst SMG
    let afk2 = Macron::new(vec![
        Press(Key::Control),
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -14000), Wait(50),

        Tap(Key::K),
        Wait(350),

        //MouseMove(0, 200), Wait(50),
        // 1
        Tap(Key::Num1), Wait(1200),
        MouseClick(Button::Left), Wait(300),
        MouseClick(Button::Left), Wait(300),
        MouseClick(Button::Left), Wait(300),
        MouseClick(Button::Left), Wait(300),
        MouseClick(Button::Left), Wait(300),

        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -13500), Wait(50),

        // 2

        Tap(Key::Num2),
        MousePress(Button::Right),
        Wait(1500),
        MouseHold(Button::Left, 400),
        MouseRelease(Button::Right),
        
        Tap(Key::W), Wait(100),
        Hold(Key::S, 500),
    ], true);

    let afk3 = Macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -16400), Wait(50),

        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(2750),
        MouseClick(Button::Left),
        Wait(1050),
        
        // walk
        Tap(Key::W), Wait(500),
        Tap(Key::S), Wait(1000),

    ], true);

    let primary = Macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -16900), Wait(50),
        Tap(Key::Num1), Wait(700),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        MouseClick(Button::Left),
        Wait(400),
        
        Tap(Key::Num2), Wait(1000),
        
        MouseClick(Button::Left),
        Wait(1050),
        Tap(Key::Num1), 
        Tap(Key::W), Wait(500),
        Tap(Key::S),
    ], true);

    // Shoot to loot + trinity + rocket
    /*
    let afk3 = Macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -16400), Wait(50),

        Tap(Key::Num2),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),
        MouseClick(Button::Left),
        Wait(2500),

        Tap(Key::Num1),
        Wait(1500),
        MouseMove(0, 24000), Wait(50),
        MouseMove(-3000, -16900),
        Wait(100),

        MouseClick(Button::Left),
        Wait(500),

        MouseMove(-2000, -1000),
        MouseClick(Button::Left),
        Wait(500),

        MouseMove(0, 200),
        MouseClick(Button::Left),
        Wait(500),

        MouseMove(5200, 500), Wait(50),
        Tap(Key::Num3),
        Wait(2000),
        MouseClick(Button::Left),
        Wait(1000),
        Tap(Key::R),
        Wait(3000),

        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17000), Wait(50),

        MouseClick(Button::Left),
        Wait(1000),
        Tap(Key::R),
        Wait(3000),
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17000), Wait(50),

        MouseClick(Button::Left),
        
        Tap(Key::Num2),
        // walk
        Tap(Key::W), Wait(500),
        Tap(Key::S), Wait(1000),

    ], true);
    */

    let icarus = Macron::new(vec![
        Press(Key::Space),
        Wait(32),
        Press(Key::N),
        Wait(20),
        Release(Key::N),
        Release(Key::Space),
    ], false);

    let mut winky_rx = winky::listen();
    loop {
        match winky_rx.recv().await.unwrap() {
            Event::Keyboard(Key::F9, true) => primary.toggle(),
            Event::Keyboard(Key::F10, true) => afk2.toggle(),
            Event::Keyboard(Key::F11, true) => afk3.toggle(),
            Event::MouseButton(Button::X1, true) => icarus.start(),
            _ => {}
        }
    }
}