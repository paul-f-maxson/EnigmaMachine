use std::{rc::Rc, cell::{Cell, RefCell}, borrow::{Borrow, BorrowMut}};

struct RotorContact {
    connected_up: Box<dyn SendUp>,
    // connected_down: Box<dyn SendDown>,
    forward: Box<RotorContact>,
    // reverse: Box<RotorContact>,
}
trait SendUp {
    fn send_up(&self);
}

trait SendDown {
    fn send_down(&self);
}

impl SendUp for RotorContact {
    fn send_up(&self) {
        self.connected_up.send_up();
    }
}

impl SendDown for RotorContact {
    fn send_down(&self) {
        todo!()
    }
}

impl RotorContact {
    fn rotate_forward(&mut self) -> Box<dyn SendUp> {

        let current_connected_up = self.connected_up;

        let new_connected_up = self.forward.rotate_forward();

        self.connected_up = new_connected_up;
        
        current_connected_up
    }
}

fn main() {
    println!("Hello, world!");
}
