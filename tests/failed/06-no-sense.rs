use endiannezz::Io;

#[derive(Io)]
#[endian(little)]
struct Foo {
    #[endian(little)]
    a: u32,
}

#[derive(Io)]
#[endian(le)]
struct Bar {
    #[endian(little)]
    a: u32,
}

fn main() {}
