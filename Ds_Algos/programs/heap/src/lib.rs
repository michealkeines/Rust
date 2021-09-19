use std::mem;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String
}

#[derive(Clone, Debug)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub device: IoTDevice
}


pub struct MessageChecker {
    pub length: usize,
    heap: Vec<Box<MessageNotification>>
}

impl MessageChecker {
    pub fn add(&mut self, notification: MessageNotification) {
        self.heap.push(Box::new(notification));
        self.length = self.heap.len();

        if self.length > 1 {
            let mut i = self.length;
            while i/2 > 0 && self.has_more_messages(i, i/2) {
                self.swap(i, i/2);
                i /= 2;
            }
        }
    }

    fn has_more_messages(&self, pos1: usize, pos2: usize) -> bool {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.no_messages >= b.no_messages
    }

    fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.heap[i].clone();
        self.heap[i] = self.heap[j].clone();
        self.heap[j] = tmp;
    }
}