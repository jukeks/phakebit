use std::sync::mpsc;

pub struct PIAChip {
    kbd_data: u8,
    kbd_control: u8,
    display_data: u8,
    display_control: u8,

    rx: mpsc::Receiver<u8>,
    tx: mpsc::Sender<u8>,
}

impl PIAChip {
    pub fn new(rx: mpsc::Receiver<u8>, tx: mpsc::Sender<u8>) -> PIAChip {
        PIAChip {
            kbd_data: 0,
            kbd_control: 0,
            display_data: 0,
            display_control: 0,
            rx,
            tx,
        }
    }
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0xD010 => {
                let data = self.kbd_data;
                self.kbd_data = 0;
                data
            }
            0xD011 => {
                let mut c = 0;
                if self.kbd_data != 0 {
                    c = 0xFF;
                } else {
                    if let Ok(v) = self.rx.try_recv() {
                        self.kbd_data = v;
                        c = 0xFF;
                    }
                }
                c
            }
            0xD012 => self.display_data,
            0xD013 => self.display_control,
            _ => 0,
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xD010 => self.kbd_data = value,
            0xD011 => {
                self.kbd_control = value;
            }
            0xD012 => {
                self.display_data = value;
                self.tx.send(value).unwrap();
                self.display_data = 0;
            }
            0xD013 => {
                self.display_control = value;
            }
            _ => (),
        }
    }
}
