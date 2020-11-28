use endiannezz::Io;

#[derive(Io, Debug, PartialEq)]
#[endian(little)]
#[repr(u32)]
enum Foo {
    Bar { x: bool } = 0xc0ffee,
    Baz { x: u32, #[endian(big)] y: i16 } = 0xdead,
}

#[test]
fn regular_enum() {
    let e1 = Foo::Bar { x: false };

    let mut vec = Vec::new();
    e1.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[0xee, 0xff, 0xc0, 0x00, 0x00]);

    let e2 = Foo::read(&mut slice).unwrap();
    assert_eq!(e1, e2);

    let e3 = Foo::Baz { x: 0x20, y: 0x10 };

    let mut vec = Vec::new();
    e3.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[0xad, 0xde, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x10]);

    let e4 = Foo::read(&mut slice).unwrap();
    assert_eq!(e3, e4);

    let mut garbage: &[u8] = &[0x00, 0x00, 0x00, 0x00];
    assert!(Foo::read(&mut garbage).is_err());
}
