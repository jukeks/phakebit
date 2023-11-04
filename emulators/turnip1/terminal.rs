use std::io::Write;
use std::sync::mpsc;
use std::thread;

use console::Term;

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {}
    }

    pub fn printer(&mut self) -> mpsc::Sender<u8> {
        let (tx, rx) = mpsc::channel();

        let mut stdout = Term::buffered_stdout();

        thread::spawn(move || loop {
            let mut c = rx.recv().unwrap();
            c = c & 0x7F; // strip high bit
            if c == 0x0D {
                // CR -> LF
                c = 0x0A;
            }
            stdout.write_fmt(format_args!("{}", c as char)).unwrap();
            stdout.flush().unwrap();
        });

        tx
    }

    pub fn reader(&mut self) -> mpsc::Receiver<u8> {
        let (tx, rx) = mpsc::channel();

        let stdout = Term::buffered_stdout();

        thread::spawn(move || {
            loop {
                if let Ok(character) = stdout.read_char() {
                    let mut c = character as u8;
                    if c == 0x0A {
                        // LF -> CR
                        c = 0x0D;
                    }
                    c = c | 0x80;
                    tx.send(c).unwrap();
                }
            }
        });

        rx
    }
}
