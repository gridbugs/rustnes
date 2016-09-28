use std::result;

pub type Address = u16;
pub type AddressDiff = u16;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BusErrorRead(Address),
    BusErrorWrite(Address),
    IllegalWrite(Address),
    UnimplementedRead(Address),
    UnimplementedWrite(Address),
}

pub trait Addressable {
    fn read8(&mut self, address: Address) -> Result<u8>;
    fn write8(&mut self, address: Address, data: u8) -> Result<()>;

    fn read16_le(&mut self, address: Address) -> Result<u16> {
        let lo = try!(self.read8(address)) as u16;
        let hi = try!(self.read8(address.wrapping_add(1))) as u16;

        Ok((hi << 8) | lo)
    }

    fn read32_le(&mut self, address: Address) -> Result<u32> {
        let a = try!(self.read8(address)) as u32;
        let b = try!(self.read8(address.wrapping_add(1))) as u32;
        let c = try!(self.read8(address.wrapping_add(2))) as u32;
        let d = try!(self.read8(address.wrapping_add(3))) as u32;

        Ok((d << 24) | (c << 16) | (b << 8) | a)
    }
}

pub trait PpuAddressable {
    fn ppu_read8(&mut self, address: Address) -> Result<u8>;
    fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()>;
}

pub trait CartridgeAddressable: Addressable + PpuAddressable {}
