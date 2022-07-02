mod macron;

use winky::{Key, Button};
use macron::Cmd;

#[tokio::main]
async fn main() {
    println!("Macro tool awaiting input");

    let afk1 = macron::new(vec![
        Cmd::MouseMove(0, 24000), Cmd::Wait(50),
        Cmd::MouseMove(0, -17100), Cmd::Wait(50),

        Cmd::Tap(Key::Num2),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(750),
        
        Cmd::MouseMove(-1200, 0), Cmd::Wait(50),

        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(700),

        Cmd::MouseMove(2200, 0), Cmd::Wait(100),

        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(350),
        Cmd::MouseClick(Button::Left), Cmd::Wait(50),

        Cmd::MouseMove(-1000, 0),

        Cmd::Tap(Key::R),
        Cmd::Tap(Key::W), Cmd::Wait(100),
        Cmd::Hold(Key::S, 450), Cmd::Wait(1400),
    ], true).await;

    let afk2 = macron::new(vec![
        Cmd::MouseMove(0, 24000), Cmd::Wait(50),
        Cmd::MouseMove(0, -15000), Cmd::Wait(50),

        Cmd::Tap(Key::Num2), Cmd::Wait(700),
        
        Cmd::MouseMove(-3000, 500),
        Cmd::MouseHold(Button::Left, 850), Cmd::Wait(600),

        Cmd::MouseMove(1500, -250),
        Cmd::MouseHold(Button::Left, 850), Cmd::Wait(600),

        Cmd::MouseMove(1500, -250),
        Cmd::MouseHold(Button::Left, 850), Cmd::Wait(600),

        Cmd::Tap(Key::R), Cmd::Wait(1500),
        Cmd::Tap(Key::W), Cmd::Wait(100),
        Cmd::Hold(Key::S, 700),   // Back
    ], true).await;

    let afk3 = macron::new(vec![
        Cmd::MouseMove(0, 24000), Cmd::Wait(50),
        Cmd::MouseMove(0, -14500), Cmd::Wait(50),

        // 1
        Cmd::Tap(Key::Num1),
        Cmd::MousePress(Button::Right),
        Cmd::Wait(900),
        Cmd::MouseClick(Button::Left),
        Cmd::MouseRelease(Button::Right),

        // walk
        Cmd::Tap(Key::W), Cmd::Wait(100),
        Cmd::Hold(Key::S, 500), Cmd::Wait(800),

        // 2
        Cmd::Tap(Key::Num2), Cmd::Wait(900),
        Cmd::MouseMove(0, 24000), Cmd::Wait(50),
        Cmd::MouseMove(0, -15000), Cmd::Wait(50),
        Cmd::MouseHold(Button::Left, 800),

        // walk
        Cmd::Tap(Key::W), Cmd::Wait(100),
        Cmd::Hold(Key::S, 400), Cmd::Wait(200),
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