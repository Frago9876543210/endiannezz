use endiannezz::Io;

#[derive(Io)]
#[repr(C)]
#[endian(big)]
struct WorksFine;

#[derive(Io)]
#[endian(big)]
#[endian(native)]
struct Nope;

fn main() {}
