use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{map, map_res},
    sequence::{preceded, terminated},
    multi::many0,
    IResult,
};

#[inline]
fn locale_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(" ")(input)
}

#[inline]
fn line_rest(input: &[u8]) -> IResult<&[u8], ()> {
    map(take_until("\n"), |_| ())(input)
}

#[inline]
fn single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(locale_name, line_rest)(input)
}

#[inline]
fn comment(input: &[u8]) -> IResult<&[u8], ()> {
    map(terminated(tag("#"), line_rest), |_| ())(input)
}

#[inline]
fn whitespace(input: &[u8]) -> IResult<&[u8], ()> {
    alt((map(multispace1, |_| ()), comment))(input)
}

#[inline]
fn hr(input: &[u8]) -> IResult<&[u8], ()> {
    map(many0(whitespace), |_| ())(input)
}

#[inline]
pub fn locale_names(input: &[u8]) -> IResult<&[u8], Vec<&str>> {
    many0(preceded(
        hr,
        map_res(single_line, |s| std::str::from_utf8(s)),
    ))(input)
}

#[test]
fn test_locale_name() {
    assert_eq!(
        locale_name(&b"zh_CN.UTF-8 "[..]),
        Ok((&b" "[..], &b"zh_CN.UTF-8"[..]))
    );
}

#[test]
fn test_line_rest() {
    assert_eq!(line_rest(&b" UTF-8\n"[..]), Ok((&b"\n"[..], ())));
}

#[test]
fn test_single_line() {
    assert_eq!(
        single_line(&b"zh_CN.UTF-8 UTF-8\n"[..]),
        Ok((&b"\n"[..], &b"zh_CN.UTF-8"[..]))
    );
}

#[test]
fn test_locale_names() {
    assert_eq!(
        locale_names(&b"#comment\n#comment2\nzh_CN.UTF-8 UTF-8\n#comment\nen_US.UTF-8 UTF-8\n"[..]),
        Ok((&b"\n"[..], vec!["zh_CN.UTF-8", "en_US.UTF-8"]))
    );
}
