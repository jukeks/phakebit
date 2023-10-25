# shadowcycle

This is a crate for emulating a 6502 CPU.

## Examples
```rust
use shadowcycle::memory::PlainMemory;
use shadowcycle::cpu::CPU;
use shadowcycle::state::CPUState;
use shadowcycle::state;

let mut memory = PlainMemory::new();
let mut cpu_state = CPUState::new(memory);
// set reset vector to program start or just set sp to same address
cpu_state.write_word(state::RESET_VECTOR_ADDR, 0x1234);
cpu_state.reset();

let mut cpu = CPU::new(cpu_state);
cpu.execute(100000);
```

## Memory mapped IO
The memory trait is used to implement memory mapped IO. The
PlainMemory struct is a simple implementation of this trait.

### Example

```rust
use shadowcycle::memory::Memory;

struct IOMappedMemory {
   state: [u8; 0x10000],
}

fn read_io(address: u16) -> u8 { 0 }
fn write_io(address: u16, value: u8) {}

impl Memory for IOMappedMemory {
   fn get(&self, address: u16) -> u8 {
      match address {
        0x0000..=0x1FFF => self.state[address as usize],
        0x2000..=0x3FFF => read_io(address),
        0x4000..=0xFFFF => self.state[address as usize],
      }
  }

 fn set(&mut self, address: u16, value: u8) {
     match address {
       0x0000..=0x1FFF => self.state[address as usize] = value,
       0x2000..=0x3FFF => write_io(address, value),
       0x4000..=0xFFFF => self.state[address as usize] = value,
     }
  }
}
````
