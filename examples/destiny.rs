use winky::{Key, Button};
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
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17200), Wait(50),

        Tap(Key::K),
        Wait(350),

        //MouseMove(0, 200), Wait(50),
        // 1
        Tap(Key::Num1),
        Wait(900),
        MouseHold(Button::Left, 600),

        MouseMove(-750, -200), Wait(50),
        MouseHold(Button::Left, 700),

        MouseMove(-750, -200), Wait(50),
        MouseHold(Button::Left, 600),

        MouseMove(2500, 0), Wait(50),
        MouseHold(Button::Left, 600),

        MouseMove(-1000, 0), Wait(50),

        // walk
        /*
        Tap(Key::R),
        Tap(Key::W), Wait(100),
        Hold(Key::S, 500), Wait(800),

        // 2
        Tap(Key::Num2), Wait(700),
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17000), Wait(50),
        MouseHold(Button::Left, 400),
        */
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

    let mut key_rx = winky::listen();
    loop {
        let (code, down) = key_rx.recv().await.unwrap();
        match code {
            Key::F9 if down => primary.toggle(),
            Key::F10 if down => afk2.toggle(),
            Key::F11 if down => afk3.toggle(),
            _ => {}
        }
    }
}