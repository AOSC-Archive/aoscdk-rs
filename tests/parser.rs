use nom::{
    bytes::complete::take_until,
    character::complete::{char, space1},
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};

#[inline]
fn key_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(":")(input)
}

#[inline]
fn separator(input: &[u8]) -> IResult<&[u8], ()> {
    map(tuple((char(':'), space1)), |_| ())(input)
}

#[inline]
fn single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("\n")(input)
}

#[inline]
fn key_value(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    separated_pair(key_name, separator, single_line)(input)
}

// tests
#[test]
fn test_key_name() {
    let test = &b"name: value"[..];
    assert_eq!(key_name(test), Ok((&b": value"[..], &b"name"[..])));
}

#[test]
fn test_seperator() {
    let test = &b": value"[..];
    let test_2 = &b": \tvalue"[..];
    assert_eq!(separator(test), Ok((&b"value"[..], ())));
    assert_eq!(separator(test_2), Ok((&b"value"[..], ())));
}

#[test]
fn test_single_line() {
    let test = &b"value\n"[..];
    let test_2 = &b"value\t\r\n"[..];
    let test_3 = &b"value \x23\xff\n"[..];
    assert_eq!(single_line(test), Ok((&b"\n"[..], &b"value"[..])));
    assert_eq!(single_line(test_2), Ok((&b"\n"[..], &b"value\t\r"[..])));
    assert_eq!(
        single_line(test_3),
        Ok((&b"\n"[..], &b"value \x23\xff"[..]))
    );
}

#[test]
fn test_key_value() {
    let test = &b"name1: value\n"[..];
    let test_2 = &b"name2: value\t\r\n"[..];
    let test_3 = &b"name3: value \x23\xff\n"[..];
    assert_eq!(
        key_value(test),
        Ok((&b"\n"[..], (&b"name1"[..], &b"value"[..])))
    );
    assert_eq!(
        key_value(test_2),
        Ok((&b"\n"[..], (&b"name2"[..], &b"value\t\r"[..])))
    );
    assert_eq!(
        key_value(test_3),
        Ok((&b"\n"[..], (&b"name3"[..], &b"value \x23\xff"[..])))
    );
}
