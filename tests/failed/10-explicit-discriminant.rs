use endiannezz::Io;

#[derive(Io)]
#[endian(big)]
#[repr(u8)]
enum Foo {
    Bar = 0xde,
    Baz,
}

fn main() {}
