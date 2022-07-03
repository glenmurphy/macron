mod macron;

use winky::{Key, Button};
use macron::Cmd::{*};

#[tokio::main]
async fn main() {
    println!("Macro tool awaiting input");

    let afk1 = macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -17100), Wait(50),

        Tap(Key::Num2),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
        MouseClick(Button::Left), Wait(350),
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
        Hold(Key::S, 450), Wait(1400),
    ], true).await;

    let afk2 = macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -15000), Wait(50),

        Tap(Key::Num2), Wait(700),
        
        MouseMove(-3000, 500),
        MouseHold(Button::Left, 850), Wait(600),

        MouseMove(1500, -250),
        MouseHold(Button::Left, 850), Wait(600),

        MouseMove(1500, -250),
        MouseHold(Button::Left, 850), Wait(600),

        Tap(Key::R), Wait(1500),
        Tap(Key::W), Wait(100),
        Hold(Key::S, 700),   // Back
    ], true).await;

    let afk3 = macron::new(vec![
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -14500), Wait(50),

        // 1
        Tap(Key::Num1),
        MousePress(Button::Right),
        Wait(900),
        MouseClick(Button::Left),
        MouseRelease(Button::Right),

        // walk
        Tap(Key::W), Wait(100),
        Hold(Key::S, 500), Wait(800),

        // 2
        Tap(Key::Num2), Wait(900),
        MouseMove(0, 24000), Wait(50),
        MouseMove(0, -15000), Wait(50),
        MouseHold(Button::Left, 800),

        // walk
        Tap(Key::W), Wait(100),
        Hold(Key::S, 400), Wait(200),
    ], true).await;

    let mut key_rx = winky::listen();
    loop {
        tokio::select! {
            Some((code, down)) = key_rx.recv() => {
                if code == Key::F9 as u32 && down {
                    let _ = afk1.toggle();
                }
                else if code == Key::F10 as u32 && down {
                    let _ = afk2.toggle();
                }
                else if code == Key::F11 as u32 && down {
                    let _ = afk3.toggle();
                }
            }
        }
    }
}