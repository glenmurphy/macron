use winky::{self, Button, Key};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel, UnboundedReceiver};
use std::{collections::HashSet};

static TAP_WAIT: u64 = 50;
//static MAX_INDIVIDUAL_WAIT: u64 = 10000;

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
    tx: UnboundedSender<Control>,
}

#[allow(unused)]
impl MacronInterface {
    pub fn start(&self) {
        let _ = self.tx.send(Control::Start);
    }
    pub fn stop(&self) {
        let _ = self.tx.send(Control::Stop);
    }
    pub fn toggle(&self) {
        let _ = self.tx.send(Control::Toggle);
    }
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum Control {
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
            /*/
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
            */
            _ => {
                let _ = cmds.push(*cmd);
            }
        }
    }
    cmds
}

struct MacronRunner {
    cmds : Vec<Cmd>,
    control_rx : UnboundedReceiver<Control>,
    cycle : bool,
    keys_down: HashSet<Key>,
    buttons_down: HashSet<Button>,
}

impl MacronRunner {
    pub fn new(cmd_list: Vec<Cmd>, cycle: bool, control_rx: UnboundedReceiver<Control>) -> Self {
        let cmds = flatten(&cmd_list);
        MacronRunner {
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
        let _ = winky::press(key);
        self.keys_down.insert(key);
    }

    fn release(&mut self, key: Key) {
        let _ = winky::release(key);
        self.keys_down.remove(&key);
    }

    fn mouse_move(&self, x: i32, y: i32) {
        winky::mouse_move(x, y);
    }

    fn mouse_press(&mut self, button: Button) {
        let _ = winky::button_press(button);
        self.buttons_down.insert(button);
    }

    fn mouse_release(&mut self, button: Button) {
        let _ = winky::button_release(button);
        self.buttons_down.remove(&button);
    }

    /// Sleeps for 'time', but if interrupted by a stop message, returns
    /// that message. Used inside the run loop.
    async fn wait(&mut self, time: u64) -> Option<Control> {
        let sleep = tokio::time::sleep(std::time::Duration::from_millis(time));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                Some(msg) = self.control_rx.recv() => {
                    match msg {
                        Control::Stop | Control::Toggle => return Some(msg),
                        _ => {}
                    }
                },
                () = &mut sleep => { return None }
            }
        }
    }

    async fn run(&mut self) {
        if self.cmds.len() == 0 { return; }

        let mut index = 0;
        println!("Starting");
        loop {
            match self.control_rx.try_recv() {
                Ok(Control::Stop) | Ok(Control::Toggle) => break,
                _ => {}
            }
            match self.cmds.get(index).unwrap().clone() {
                Cmd::Press(key) => self.press(key),
                Cmd::Release(key) => self.release(key),
                Cmd::MouseMove(x, y) => self.mouse_move(x, y),
                Cmd::MousePress(button) => self.mouse_press(button),
                Cmd::MouseRelease(button) => self.mouse_release(button),
                Cmd::Wait(time) => {
                    match self.wait(time).await {
                        Some(Control::Stop) | Some(Control::Toggle) => break,
                        _ => {}
                    }
                },
                _ => panic!("unexpected command")
            }
            if index == self.cmds.len() - 1 && !self.cycle {
                break;
            }
            index = (index + 1) % self.cmds.len();
        }

        println!("Stopped");
        self.stop();
    }

    pub async fn start(&mut self) {
        loop {
            let msg = self.control_rx.recv().await;
            match msg {
                Some(Control::Start) | Some(Control::Toggle) => self.run().await,
                _ => { }
            }
        }
    }
}

pub async fn new(cmds: Vec<Cmd>, cycle: bool) -> MacronInterface {
    let (control_tx, control_rx) = unbounded_channel();
    tokio::spawn(async move {
        MacronRunner::new(cmds, cycle, control_rx).start().await;
    });
    MacronInterface { tx: control_tx }
}