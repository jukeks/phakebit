mod emulator;
mod memory;
mod pia;
mod terminal;

use argh::FromArgs;
use std::fs;

use emulator::Emulator;
use terminal::Terminal;

#[derive(FromArgs)]
/// Command line arguments for turnip1
struct CLIParams {
    /// path for binary to run
    #[argh(option, short = 'b')]
    binary_path: String,

    /// address to load the binary (16 bit hexadecimal, e.g. 0x5F3C)
    #[argh(option, short = 'l', from_str_fn(parse_hex))]
    load_address: u16,

    /// sets RESET_VECTOR (16 bit hexadecimal, e.g. 0x5F3C); defaults to the load address if not specified
    #[argh(option, short = 's', from_str_fn(parse_hex))]
    start_address: Option<u16>,
}

/// Parses a hexadecimal string into a u16
fn parse_hex(value: &str) -> Result<u16, String> {
    u16::from_str_radix(value.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}

pub fn main() {
    let params: CLIParams = argh::from_env();
    let start_address = match params.start_address {
        Some(addr) => addr,
        None => params.load_address,
    };

    let binary = fs::read(&params.binary_path).expect("Unable to read binary");

    let mut terminal = Terminal::new();

    let dsp_tx = terminal.printer();
    let kbd_rx = terminal.reader();

    let emu = Emulator::new(kbd_rx, dsp_tx);
    emu.execute_program(binary, params.load_address, start_address);
}
