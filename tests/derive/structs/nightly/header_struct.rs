use endiannezz::{Io, HardcodedPayload};

#[derive(Debug, Default, PartialEq)]
struct Header;

impl HardcodedPayload for Header {
    type Buf = [u8; 8];
    const PAYLOAD: Self::Buf = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
}

#[test]
fn header_struct() {
    let s1 = Header;

    let mut vec = Vec::new();
    s1.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);

    let s2 = Header::read(&mut slice).unwrap();
    assert_eq!(s1, s2);

    let mut garbage: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x00];
    assert!(Header::read(&mut garbage).is_err());
}
