use endiannezz::Io;

#[derive(Io, Debug, PartialEq)]
#[endian(big)]
struct ParseMe;

#[test]
fn unit_struct() {
    let s1 = ParseMe;

    let mut vec = Vec::new();
    s1.write(&mut vec).unwrap();

    let mut slice = vec.as_slice();
    assert_eq!(slice, &[]);

    let s2 = ParseMe::read(&mut slice).unwrap();
    assert_eq!(s1, s2);
}
