use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, terminated, tuple},
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
    many0(preceded(hr, map_res(single_line, std::str::from_utf8)))(input)
}

#[inline]
fn zone1970_single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, _, _, _, tz, _, _)) = tuple((
        take_until("\t"),
        multispace1,
        take_until("\t"),
        multispace1,
        take_while1(|c| c != b'\t' && c != b'\n'),
        take_until("\n"),
        line_rest,
    ))(input)?;

    Ok((input, tz))
}

#[inline]
pub fn list_zoneinfo(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let (input, result) = many0(preceded(
        hr,
        map_res(zone1970_single_line, std::str::from_utf8),
    ))(input)?;

    Ok((input, result.into_iter().map(|x| x.into()).collect()))
}

#[test]
fn test_zone1970_single_line() {
    use std::str;

    // no comments item on zone1970.tab
    assert_eq!(
        str::from_utf8(
            zone1970_single_line(&b"AD\t+4230+00131\tEurope/Andorra8\n"[..])
                .unwrap()
                .1
        )
        .unwrap(),
        "Europe/Andorra8"
    );

    // have comments item on zone1970.tab
    assert_eq!(
        str::from_utf8(
            zone1970_single_line(&b"AQ\t-6617+1103\tAntarctica/Casey\tCasey\n"[..])
                .unwrap()
                .1
        )
        .unwrap(),
        "Antarctica/Casey"
    );
}

#[test]
fn test_list_zoneinfo() {
    let buf = &b"#commit1\tcommit2\t\na\tb\tc/c\nd\te\tf/f\tg\n#commit3\nh\ti\tj/j\n"[..];
    assert_eq!(list_zoneinfo(buf).unwrap().1, vec!["c/c", "f/f", "j/j"]);
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
