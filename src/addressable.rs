use std::result;

pub type Address = u16;
pub type AddressDiff = u16;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BusErrorRead(Address),
    BusErrorWrite(Address),
    IllegalWrite(Address),
    IllegalRead(Address),
    UnimplementedRead(Address),
    UnimplementedWrite(Address),
}

// Note that defaults are sane for normal memory only
pub trait Addressable {
    fn write8(&mut self, address: Address, data: u8) -> Result<()>;
    fn read8(&mut self, address: Address) -> Result<u8>;
    fn read8_pure(&mut self, address: Address) -> Result<u8> {
        self.read8(address)
    }
    fn read8_side_effects(&mut self, address: Address) -> Result<()> {
        try!(self.read8(address));
        Ok(())
    }
    fn read16_le(&mut self, address: Address) -> Result<u16> {
        let lo = try!(self.read8(address)) as u16;
        let hi = try!(self.read8(address.wrapping_add(1))) as u16;

        Ok((hi << 8) | lo)
    }
    fn read16_le_pure(&mut self, address: Address) -> Result<u16> {
        let lo = try!(self.read8_pure(address)) as u16;
        let hi = try!(self.read8_pure(address.wrapping_add(1))) as u16;

        Ok((hi << 8) | lo)
    }
}

pub trait PpuAddressable {
    fn ppu_read8(&mut self, address: Address) -> Result<u8>;
    fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()>;
}

pub trait CartridgeAddressable: Addressable + PpuAddressable {}
