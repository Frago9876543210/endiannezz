use endiannezz::Io;

#[derive(Io)]
#[endian(invalid)]
struct Foo;

fn main() {}
