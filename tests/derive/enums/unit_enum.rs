use endiannezz::Io;

#[derive(Io, Debug, PartialEq, Copy, Clone)]
#[endian(little)]
#[repr(u32)]
enum Foo {
    Bar = 0xc0ffee,
    Baz = 0xdead,
    Qux = 0xfeed,
}

#[test]
fn unit_enum() {
    let e1 = Foo::Bar;

    let mut vec = Vec::new();
    e1.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[0xee, 0xff, 0xc0, 0x00]);

    let e2 = Foo::read(&mut slice).unwrap();
    assert_eq!(e1, e2);

    let mut garbage: &[u8] = &[0x00, 0x00, 0x00, 0x00];
    assert!(Foo::read(&mut garbage).is_err());
}
