use addressable::Address;

pub const MIRROR_SIZE: Address = 0x1000;

pub trait Mirror {
    fn mirror(address: Address) -> Address;
}

#[derive(Debug)]
pub struct HorizontalMirror;

#[derive(Debug)]
pub struct VerticalMirror;

impl Mirror for HorizontalMirror {
    fn mirror(address: Address) -> Address {
        address / 2
    }
}

impl Mirror for VerticalMirror {
    fn mirror(address: Address) -> Address {
        address % (MIRROR_SIZE / 2)
    }
}
