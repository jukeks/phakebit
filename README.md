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
// set reset vector to program start or just point `cpu_state.sp` address
cpu_state.write_word(state::RESET_VECTOR_ADDR, 0x1234);
cpu_state.reset();

let mut cpu = CPU::new(cpu_state);
cpu.execute(100000);
```

## Memory maps
The `Memory` trait is used to implement memory. The PlainMemory struct is a
simple implementation of this trait without any mappings. If you want to map
parts of the memory to IO or ROM, you can implement the `Memory` trait for
your own struct.

### Example

```rust
use shadowcycle::memory::Memory;

struct MemoryMappedIO {
   state: [u8; 0x10000],
}

fn read_io(address: u16) -> u8 { 0 }
fn write_io(address: u16, value: u8) {}

impl Memory for MemoryMappedIO {
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
```

## Instrumentation
The `Trace` struct is used to instrument the CPU. It contains the state of
the CPU _after_ executing the instruction. The `CPU::step()` method returns
a `Trace`.

```
PC   Op Oper   Disassembly   |A  X  Y  SP|NVDIZC|C
357E A5 0D     LDA $0D       |00 0E FF FC|011010|3
3580 71 56     ADC ($56),Y   |00 0E FF FC|001010|5
3582 08        PHP           |00 0E FF FB|001010|3
3583 C5 0F     CMP $0F       |00 0E FF FB|001011|3
3585 D0 FE     BNE $3585     |00 0E FF FB|001011|2
3587 68        PLA           |3A 0E FF FC|001001|4
```
