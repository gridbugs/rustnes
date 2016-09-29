use cpu_memory_layout::NesCpuMemoryLayout;
use addressable::{Address, Addressable, Result};

pub struct MemoryWrite {
    address: Address,
    data: u8,
}

pub struct NesCpuMemoryLayoutBuffer<'a, C: 'a + Addressable> {
    memory: NesCpuMemoryLayout<'a, C>,
    writes: &'a mut Vec<MemoryWrite>,
    reads: &'a mut Vec<Address>,
}

impl<'a, C: 'a + Addressable> NesCpuMemoryLayoutBuffer<'a, C> {
    pub fn new(memory: NesCpuMemoryLayout<'a, C>,
               writes: &'a mut Vec<MemoryWrite>,
               reads: &'a mut Vec<Address>)
               -> Self {

        NesCpuMemoryLayoutBuffer {
            memory: memory,
            writes: writes,
            reads: reads,
        }
    }

    pub fn apply(&mut self) -> Result<()> {
        for address in self.reads.drain(..) {
            try!(self.memory.read8_side_effects(address));
        }
        for write in self.writes.drain(..) {
            try!(self.memory.write8(write.address, write.data));
        }

        Ok(())
    }
}

impl<'a, C: 'a + Addressable> Addressable for NesCpuMemoryLayoutBuffer<'a, C> {
    fn read8(&mut self, address: Address) -> Result<u8> {
        self.reads.push(address);
        self.memory.read8_pure(address)
    }
    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        self.writes.push(MemoryWrite {
            address: address,
            data: data,
        });
        Ok(())
    }
}
