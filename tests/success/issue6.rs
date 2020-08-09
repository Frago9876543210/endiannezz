use endiannezz::Io;

#[derive(Io)]
#[endian(_)]
struct DefaultMacroCase {
    compiles: bool,
}

fn main() {}
