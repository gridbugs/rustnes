macro_rules! bit {
    ($x:expr) => {1 << $x}
}

macro_rules! mask {
    ($x:expr) => { bit!($x) - 1 }
}
