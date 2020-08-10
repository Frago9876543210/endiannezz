use endiannezz::Io;

#[derive(Io)]
#[endian(big)]
enum Foo {
    Bar,
    Baz,
}

fn main() {}
