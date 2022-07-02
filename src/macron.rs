use winky::{button_press, button_release, mouse_move, press, release, Button, Key};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use std::collections::HashSet;
use tokio_stream::{self as stream, StreamExt};

static TAP_WAIT: u64 = 50;

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub enum Cmd {
    Press(Key),
    Release(Key),
    MouseMove(i32, i32),
    MousePress(Button),
    MouseRelease(Button),
    Wait(u64),
    
    // Compound commands:
    Tap(Key),
    Hold(Key, u64),
    MouseClick(Button),
    MouseHold(Button, u64),
}

pub struct MacronInterface {
    tx: UnboundedSender<MacronControl>,
}

#[allow(unused)]
impl MacronInterface {
    pub fn start(&self) {
        let _ = self.tx.send(MacronControl::Start);
    }
    pub fn stop(&self) {
        let _ = self.tx.send(MacronControl::Stop);
    }
    pub fn toggle(&self) {
        let _ = self.tx.send(MacronControl::Toggle);
    }
}

#[allow(unused)]
pub enum MacronControl {
    Start,
    Stop,
    Toggle,
}

fn flatten(cmd_list: &Vec<Cmd>) -> Vec<Cmd> {
    // Split any compound commands into single commands
    let mut cmds: Vec<Cmd> = Vec::new();
    for cmd in cmd_list {
        match cmd {
            Cmd::Tap(key) => {
                cmds.push(Cmd::Press(*key));
                cmds.push(Cmd::Wait(TAP_WAIT));
                cmds.push(Cmd::Release(*key));
            }
            Cmd::Hold(key, ms) => {
                cmds.push(Cmd::Press(*key));
                cmds.push(Cmd::Wait(*ms));
                cmds.push(Cmd::Release(*key));
            }
            Cmd::MouseClick(button) => {
                cmds.push(Cmd::MousePress(*button));
                cmds.push(Cmd::Wait(TAP_WAIT));
                cmds.push(Cmd::MouseRelease(*button));
            }
            Cmd::MouseHold(button, ms) => {
                cmds.push(Cmd::MousePress(*button));
                cmds.push(Cmd::Wait(*ms));
                cmds.push(Cmd::MouseRelease(*button));
            }
            _ => {
                let _ = cmds.push(*cmd);
            }
        }
    }
    cmds
}


pub async fn new(cmd_list: Vec<Cmd>, cycle: bool) -> MacronInterface {
    let (control_tx, mut control_rx) = unbounded_channel();
    let cmds = flatten(&cmd_list);

    tokio::spawn(async move {
        let mut run_cmds_stream = stream::iter(&cmds);
        let mut running = false;
        let mut keys_down: HashSet<Key> = HashSet::new();
        let mut buttons_down: HashSet<Button> = HashSet::new();

        loop {
            tokio::select! {
                Some(msg) = control_rx.recv() => {
                    match msg {
                        MacronControl::Start => {
                            println!("starting");
                            running = true;
                            run_cmds_stream = stream::iter(&cmds);
                        },
                        MacronControl::Stop => {
                            println!("stopping");
                            running = false;

                            for key in keys_down.iter() { release(*key); }
                            for button in buttons_down.iter() { button_release(*button); }
                            keys_down.clear();
                            buttons_down.clear();
                        },
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
                    }
                }
                maybe_c = run_cmds_stream.next(), if running => {
                    if let Some(c) = maybe_c {
                        //println!("running cmd: {:?}", &c);
                        match c {
                            Cmd::Press(key) => {
                                press(*key);
                                keys_down.insert(*key);
                            }
                            Cmd::Release(key) => {
                                release(*key);
                                keys_down.remove(&key);
                            }
                            Cmd::MouseMove(x, y) => {
                                mouse_move(*x, *y);
                            }
                            Cmd::MousePress(button) => {
                                button_press(*button);
                                buttons_down.insert(*button);
                            }
                            Cmd::MouseRelease(button) => {
                                button_release(*button);
                                buttons_down.remove(&button);
                            }
                            Cmd::Wait(time) => {
                                // ideally we would pause the stream and use a timer
                                // so the channels can continue working
                                std::thread::sleep(std::time::Duration::from_millis(*time));
                            },
                            _ => {
                                panic!("unexpected command: {:?}", c);
                            }
                        }
                    } else if cycle {
                        // Run out of commands, loop
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