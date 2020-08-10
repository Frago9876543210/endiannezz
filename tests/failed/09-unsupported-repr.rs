use endiannezz::Io;

#[derive(Io)]
#[endian(big)]
#[repr(C)]
enum Foo {
    Bar,
    Baz,
}

fn main() {}
