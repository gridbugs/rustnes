use std::result;

pub type Address = u16;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BusErrorRead(Address),
    BusErrorWrite(Address),
    IllegalWrite(Address),
}

pub trait CpuAddressable {
    fn read(&mut self, address: Address) -> Result<u8>;
    fn write(&mut self, address: Address, data: u8) -> Result<()>;
}

pub trait PpuAddressable {
    fn read(&mut self, address: Address) -> Result<u8>;
    fn write(&mut self, address: Address, data: u8) -> Result<()>;
}
