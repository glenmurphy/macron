mod macron;

use winky::{Key, Button};
use macron::Cmd;

#[tokio::main]
async fn main() {
    println!("Macro tool awaiting input");
    //println!("{:?}", screen::detect(255, 0, 0, 10));

    // Sunshot spam
    let afk1 = macron::new(vec![
        Cmd::MouseMove(0, 24000),
        Cmd::Sleep(50),
        Cmd::MouseMove(0, -17500),
        Cmd::Sleep(50),

        Cmd::Tap(Key::Num2),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        
        Cmd::MouseMove(-1000, 0),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::MouseMove(2000, 0),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::Click(Button::Left),
        Cmd::Sleep(400),
        Cmd::MouseMove(-1000, 0),

        Cmd::Tap(Key::W),           // Forward
        Cmd::Sleep(100),

        Cmd::Tap(Key::R),
        Cmd::Press(Key::S),   // Back
        Cmd::Sleep(450),
        Cmd::Release(Key::S),
        Cmd::Sleep(1000),
    ]).await;

    // Mostly 2, single hold of 3
    let afk2 = macron::new(vec![
        Cmd::MouseMove(0, 24000),
        Cmd::Sleep(50),
        Cmd::MouseMove(0, -15000),
        Cmd::Sleep(50),

        Cmd::Tap(Key::Num2),
        Cmd::Sleep(700),

        Cmd::MouseMove(-3000, 500),
        Cmd::ButtonPress(Button::Left),
        Cmd::Sleep(850),
        Cmd::ButtonRelease(Button::Left),
        Cmd::Sleep(600),

        Cmd::MouseMove(1500, -250),
        Cmd::ButtonPress(Button::Left),
        Cmd::Sleep(850),
        Cmd::ButtonRelease(Button::Left),
        Cmd::Sleep(600),

        Cmd::MouseMove(1500, -250),
        Cmd::ButtonPress(Button::Left),
        Cmd::Sleep(800),
        Cmd::ButtonRelease(Button::Left),
        Cmd::Sleep(600),

        Cmd::Tap(Key::R),
        Cmd::Sleep(1500),
        
        Cmd::Tap(Key::W),           // Forward
        Cmd::Sleep(100),
        Cmd::Engage(Key::S, 700),   // Back
    ]).await;

    // Swap between 1 and 3
    let afk3 = macron::new(vec![
        Cmd::MouseMove(0, 24000),
        Cmd::Sleep(50),
        Cmd::MouseMove(0, -14500),
        Cmd::Sleep(50),

        // 1
        Cmd::Tap(Key::Num1),
        Cmd::ButtonPress(Button::Right),
        Cmd::Sleep(900),
        Cmd::Click(Button::Left),
        Cmd::ButtonRelease(Button::Right),

        // walk
        Cmd::Tap(Key::W),           // Forward
        Cmd::Sleep(100),
        Cmd::Engage(Key::S, 500),   // Back
        Cmd::Sleep(800),

        // 2
        Cmd::Tap(Key::Num2),
        Cmd::Sleep(900),
        Cmd::MouseMove(0, 24000),
        Cmd::Sleep(50),
        Cmd::MouseMove(0, -15000),
        Cmd::Sleep(50),
        Cmd::ButtonPress(Button::Left),
        Cmd::Sleep(800),
        Cmd::ButtonRelease(Button::Left),

        Cmd::Tap(Key::W),           // Forward
        Cmd::Sleep(100),
        Cmd::Engage(Key::S, 400),   // Back
        Cmd::Sleep(200),

        // walk
        Cmd::Tap(Key::W),           // Forward
        Cmd::Sleep(100),
        Cmd::Engage(Key::S, 400),   // Back
        Cmd::Sleep(200),
    ]).await;

    let mut key_rx = winky::listen();
    loop {
        tokio::select! {
            Some((code, down)) = key_rx.recv() => {
                //println!("{:?}, {}", code, down);
                if code == Key::F9 as u32 && down {
                    println!("Toggling macro");
                    let _ = afk1.toggle();
                }
                else if code == Key::F10 as u32 && down {
                    println!("Toggling macro");
                    let _ = afk2.toggle();
                }
                else if code == Key::F11 as u32 && down {
                    println!("Toggling macro");
                    let _ = afk3.toggle();
                }
            }
        }
    }
}