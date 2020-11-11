#[derive(Debug)]
pub struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    to: u64,
    content: String,
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox { messages: vec![] }
    }

    pub fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

pub struct GroundStation {
    mailbox: Mailbox,
    satellites: Vec<CubeSat>,
}

impl GroundStation {
    pub fn new() -> GroundStation {
        GroundStation {
            mailbox: Mailbox::new(),
            satellites: vec![],
        }
    }

    pub fn connect(&mut self, sat_id: u64) -> CubeSat {
        let sat = CubeSat { id: sat_id };
        self.satellites.push(sat);
        sat
    }

    pub fn disconnect(&mut self) {
        self.mailbox.clear()
    }

    pub fn send(&mut self, msg: Message) {
        self.mailbox.post(msg);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CubeSat {
    id: u64,
}

impl CubeSat {
    pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn main() {
    let mut houston = GroundStation::new();

    for sat_id in vec![1, 2, 3] {
        houston.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        houston.send(msg);
    }

    for sat in &houston.satellites {
        let msg = &sat.recv(&mut houston.mailbox);
        println!("{:?}: {:?}", sat, msg);
        println!("{:?}", &houston.mailbox);
    }
}
