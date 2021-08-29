use endiannezz::Io;

#[derive(Io, Debug, PartialEq)]
#[endian(big)]
struct ParseMe {
    works: bool,
    data: u32,
    #[endian(little)]
    extra: i16,
    u16_be: [u16; 2],
    #[endian(little)]
    u16_le: [u16; 2],
}

#[test]
fn regular_struct() {
    let s1 = ParseMe {
        works: true,
        data: 10,
        extra: 20,
        u16_be: [0xff; 2],
        u16_le: [0xff; 2],
    };

    let mut vec = Vec::new();
    s1.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[1, 0, 0, 0, 10, 20, 0, 0x00, 0xff, 0x00, 0xff, 0xff, 0x00, 0xff, 0x00]);

    let s2 = ParseMe::read(&mut slice).unwrap();
    assert_eq!(s1, s2);
}
