use winky::{button_press, button_release, mouse_move, press, release, Button, Key};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use std::collections::HashSet;
use tokio_stream::{self as stream, StreamExt};

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub enum Cmd {
    Press(Key),
    Release(Key),
    Tap(Key),
    Engage(Key, u64),
    Sleep(u64),
    MouseMove(i32, i32),
    Click(Button),
    ButtonPress(Button),
    ButtonRelease(Button),
    Aim(u64),
    Scan(i32, i32, u64)
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn aim_at(x: usize, y: usize, sensitivity: f32) {
    let mx = ((x as f32 + 100f32) - (2560 / 2) as f32) * sensitivity;
    let my = ((1440 / 2) as f32 - (y as f32 + 60f32)) * sensitivity;
    mouse_move(mx as i32, my as i32);
}

pub struct MacronInterface {
    tx: UnboundedSender<MacronControl>,
}

#[allow(unused)]
impl MacronInterface {
    pub fn toggle(&self) {
        let _ = self.tx.send(MacronControl::Toggle);
    }

    pub fn set_aim(&self, x: usize, y: usize) {
        let _ = self.tx.send(MacronControl::SetAim(x, y));
    }
}

#[allow(unused)]
pub enum MacronControl {
    Toggle,
    SetAim(usize, usize)
}

struct Aim {
    start : std::time::Instant,
    duration : u128,
}

impl Aim {
    pub fn reset(&mut self, duration: u128) {
        self.start = std::time::Instant::now();
        self.duration = duration;
    }

    pub fn is_aiming(&self) -> bool {
        return self.start.elapsed().as_millis() < self.duration;
    }
}

pub async fn new(cmds: Vec<Cmd>) -> MacronInterface {
    let (control_tx, mut control_rx) = unbounded_channel();

    //let (parent_tx, parent_rx) = unbounded_channel();
    tokio::spawn(async move {
        let mut run_cmds_stream = stream::iter(&cmds);
        let mut running = false;
        let mut keys_down: HashSet<Key> = HashSet::new();
        let mut buttons_down: HashSet<Button> = HashSet::new();
        let mut aim = Aim { start : std::time::Instant::now(), duration : 0 };

        loop {
            tokio::select! {
                Some(msg) = control_rx.recv() => {
                    match msg {
                        MacronControl::Toggle => {
                            println!("toggling");
                            if running {
                                println!("stopping");
                                running = false;
                                for key in keys_down.iter() { release(*key); }
                                for button in buttons_down.iter() { button_release(*button); }
                                keys_down.clear();
                                buttons_down.clear();
                            } else {
                                println!("starting");
                                running = true;
                                run_cmds_stream = stream::iter(&cmds);
                            }
                        }
                        MacronControl::SetAim(x, y) => {
                            if aim.is_aiming() {
                                aim_at(x, y, 1.0f32);
                            }
                        }
                    }
                }
                maybe_c = run_cmds_stream.next(), if running && !aim.is_aiming() => {
                    if let Some(c) = maybe_c {
                        println!("running cmd: {:?}", &c);
                        match c {
                            Cmd::Press(key) => {
                                press(*key);
                                keys_down.insert(*key);
                            }
                            Cmd::Release(key) => {
                                release(*key);
                                keys_down.remove(&key);
                            }
                            Cmd::Tap(key) => {
                                press(*key);
                                sleep(100);
                                release(*key);
                            }
                            Cmd::Engage(key, time) => {
                                press(*key);
                                sleep(*time);
                                release(*key);
                            }
                            Cmd::Sleep(time) => sleep(*time),
                            Cmd::MouseMove(x, y) => mouse_move(*x, *y),
                            Cmd::Click(button) => {
                                button_press(*button);
                                sleep(50);
                                button_release(*button);
                            }
                            Cmd::ButtonPress(button) => {
                                button_press(*button);
                                buttons_down.insert(*button);
                            }
                            Cmd::ButtonRelease(button) => {
                                button_release(*button);
                                buttons_down.remove(&button);
                            }
                            Cmd::Aim(time) => {
                                aim.reset(*time as u128);
                            }
                            _ => {}
                        }
                    } else {
                        run_cmds_stream = stream::iter(&cmds);
                    }
                }
            }
        }
    });

    MacronInterface {
        tx : control_tx
    }
}