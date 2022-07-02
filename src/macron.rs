use winky::{button_press, button_release, mouse_move, press, release, Button, Key};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel, UnboundedReceiver};
use std::{collections::HashSet};

static TAP_WAIT: u64 = 50;
static MAX_INDIVIDUAL_WAIT: u64 = 10000;

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

#[derive(PartialEq)]
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
            Cmd::Wait(ms) => {
                // Break into smaller waits
                if *ms > MAX_INDIVIDUAL_WAIT {
                    let mut ms = *ms;
                    while ms > MAX_INDIVIDUAL_WAIT {
                        cmds.push(Cmd::Wait(MAX_INDIVIDUAL_WAIT));
                        ms -= MAX_INDIVIDUAL_WAIT;
                    }
                    cmds.push(Cmd::Wait(ms));
                }
                else {
                    cmds.push(Cmd::Wait(*ms));
                }
            }
            _ => {
                let _ = cmds.push(*cmd);
            }
        }
    }
    cmds
}

struct Macron {
    cmds : Vec<Cmd>,
    control_rx : UnboundedReceiver<MacronControl>,
    cycle : bool,
    keys_down: HashSet<Key>,
    buttons_down: HashSet<Button>,
}

impl Macron {
    pub fn new(cmd_list: Vec<Cmd>, cycle: bool, control_rx: UnboundedReceiver<MacronControl>) -> Self {
        let cmds = flatten(&cmd_list);
        Macron {
            cmds,
            control_rx,
            cycle,
            keys_down : HashSet::new(),
            buttons_down : HashSet::new(),
        }
    }

    fn clear_keys(&mut self) {
        while let Some(&key) = self.keys_down.iter().next() {
            self.release(key);
        }
    }

    fn clear_buttons(&mut self) {
        while let Some(&button) = self.buttons_down.iter().next() {
            self.mouse_release(button);
        }
    }

    fn stop(&mut self) {
        self.clear_keys();
        self.clear_buttons();
    }

    fn press(&mut self, key: Key) {
        let _ = press(key);
        self.keys_down.insert(key);
    }

    fn release(&mut self, key: Key) {
        let _ = release(key);
        self.keys_down.remove(&key);
    }

    fn mouse_press(&mut self, button: Button) {
        let _ = button_press(button);
        self.buttons_down.insert(button);
    }

    fn mouse_release(&mut self, button: Button) {
        let _ = button_release(button);
        self.buttons_down.remove(&button);
    }

    async fn wait(&mut self, time: u64) -> Option<MacronControl> {
        let sleep = tokio::time::sleep(std::time::Duration::from_millis(time));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                Some(msg) = self.control_rx.recv() => {
                    match msg {
                        MacronControl::Stop | MacronControl::Toggle => {
                            return Some(msg);
                        },
                        _ => {}
                    }
                },
                () = &mut sleep => { return None }
            }
        }
    }

    async fn run(&mut self) {
        let mut index = 0;
        if self.cmds.len() == 0 { return; }
        
        loop {
            match self.control_rx.try_recv() {
                Ok(MacronControl::Stop) | Ok(MacronControl::Toggle) => {
                    self.stop();
                    return;
                },
                _ => {}
            }
            match self.cmds.get(index).unwrap().clone() {
                Cmd::Press(key) => self.press(key),
                Cmd::Release(key) => self.release(key),
                Cmd::MouseMove(x, y) => mouse_move(x, y),
                Cmd::MousePress(button) => self.mouse_press(button),
                Cmd::MouseRelease(button) => self.mouse_release(button),
                Cmd::Wait(time) => {
                    if let Some(msg) = self.wait(time).await {
                        if msg == MacronControl::Stop || msg == MacronControl::Toggle {
                            self.stop();
                            return;
                        }
                    }
                },
                _ => panic!("unexpected command")
            }
            index = index + 1;
            if index >= self.cmds.len() {
                if self.cycle { 
                    index = 0;
                } else {
                    self.stop();
                    return;
                }
            }
        }
    }

    pub async fn start(&mut self) {
        loop {
            let msg = self.control_rx.recv().await;
            match msg {
                Some(MacronControl::Start) | Some(MacronControl::Toggle) => self.run().await,
                _ => { }
            }
        }
    }
}

pub async fn new(cmds: Vec<Cmd>, cycle: bool) -> MacronInterface {
    let (control_tx, control_rx) = unbounded_channel();
    tokio::spawn(async move {
        Macron::new(cmds, cycle, control_rx).start().await;
    });
    MacronInterface { tx: control_tx }
}