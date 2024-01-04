use std::str::Utf8Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use anyhow::Result;

#[inline]
fn line_rest(input: &[u8]) -> IResult<&[u8], ()> {
    map(take_until("\n"), |_| ())(input)
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

type Language<'a> = (&'a [u8], &'a [u8], &'a [u8]);

#[inline]
fn languagelist_single_line(input: &[u8]) -> IResult<&[u8], Language> {
    let (input, (_, _, language_english, _, language, _, _, _, _, _, locale, _, _, _)) =
        tuple((
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
        ))(input)?;

    Ok((input, (language, locale, language_english)))
}

pub fn parse_languagelist(input: &[u8]) -> IResult<&[u8], Vec<(&str, &str, &str)>> {
    let (input, result) = many0(preceded(
        hr,
        map_res(languagelist_single_line, |v| {
            Ok::<(&str, &str, &str), Utf8Error>({
                (
                    std::str::from_utf8(v.0)?,
                    std::str::from_utf8(v.1)?,
                    std::str::from_utf8(v.2)?,
                )
            })
        }),
    ))(input)?;

    Ok((input, result))
}

#[inline]
pub fn list_zoneinfo(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let (input, result) = many0(preceded(
        hr,
        map_res(zone1970_single_line, std::str::from_utf8),
    ))(input)?;

    Ok((input, result.into_iter().map(|x| x.into()).collect()))
}

#[inline]
fn mounts_single_line(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    let (input, (dev, _, mount_path, _, _)) = tuple((
        take_until(" "),
        multispace1,
        take_until(" "),
        line_rest,
        line_rest,
    ))(input)?;

    Ok((input, (dev, mount_path)))
}

fn mounts_to_turple<'a>(input: (&'a [u8], &'a [u8])) -> Result<(&'a str, &'a str)> {
    let x = std::str::from_utf8(input.0)?;
    let y = std::str::from_utf8(input.1)?;

    Ok((x, y))
}

#[inline]
pub fn list_mounts(input: &[u8]) -> IResult<&[u8], Vec<(&str, &str)>> {
    let (input, result) =
        many0(preceded(hr, map_res(mounts_single_line, mounts_to_turple)))(input)?;

    Ok((input, result))
}

#[test]
fn test_languagelist_single_line() {
    let s = "zh_CN;Chinese (Simplified);中文(简体);3;CN;zh_CN.UTF-8;zh_CN:zh;";
    let res = languagelist_single_line(s.as_bytes());
    let res = res.unwrap();
    let input = res.0;
    let (lang, locale, language_english) = res.1;
    let lang = std::str::from_utf8(lang).unwrap();
    let locale = std::str::from_utf8(locale).unwrap();
    let lang_english = std::str::from_utf8(language_english).unwrap();

    assert_eq!(input, &[] as &[u8]);
    assert_eq!(lang, "中文(简体)");
    assert_eq!(locale, "zh_CN.UTF-8");
    assert_eq!(lang_english, "Chinese (Simplified)");
}

#[test]
fn test_parse_languagelist() {
    let output = parse_languagelist("zh_CN;Chinese (Simplified);中文(简体);3;CN;zh_CN.UTF-8;zh_CN:zh;\nzh_TW;Chinese (Traditional);中文(繁體);3;TW;zh_TW.UTF-8;zh_TW:zh;\n".as_bytes());
    assert_eq!(
        output,
        Ok((
            &[10_u8][..],
            vec![
                ("中文(简体)", "zh_CN.UTF-8", "Chinese (Simplified)"),
                ("中文(繁體)", "zh_TW.UTF-8", "Chinese (Traditional)")
            ],
        ))
    )
}

#[test]
fn test_mounts_single_line() {
    let s = mounts_single_line(b"/dev/nvme0n1p1 /efi vfat rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,utf8,errors=remount-ro 0 0\n").unwrap().1;
    let dev = std::str::from_utf8(s.0).unwrap();
    let mount_path = std::str::from_utf8(s.1).unwrap();

    assert_eq!(dev, "/dev/nvme0n1p1");
    assert_eq!(mount_path, "/efi");
}

#[test]
fn test_list_mounts() {
    let mounts = list_mounts(&br#"proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
devtmpfs /dev devtmpfs rw,nosuid,size=4096k,nr_inodes=65536,mode=755 0 0
securityfs /sys/kernel/security securityfs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /dev/shm tmpfs rw,nosuid,nodev 0 0
devpts /dev/pts devpts rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000 0 0
tmpfs /run tmpfs rw,nosuid,nodev,size=3127908k,nr_inodes=819200,mode=755 0 0
tmpfs /sys/fs/cgroup tmpfs ro,nosuid,nodev,noexec,size=4096k,nr_inodes=1024,mode=755 0 0
cgroup2 /sys/fs/cgroup/unified cgroup2 rw,nosuid,nodev,noexec,relatime,nsdelegate 0 0
cgroup /sys/fs/cgroup/systemd cgroup rw,nosuid,nodev,noexec,relatime,xattr,name=systemd 0 0
pstore /sys/fs/pstore pstore rw,nosuid,nodev,noexec,relatime 0 0
efivarfs /sys/firmware/efi/efivars efivarfs rw,nosuid,nodev,noexec,relatime 0 0
bpf /sys/fs/bpf bpf rw,nosuid,nodev,noexec,relatime,mode=700 0 0
cgroup /sys/fs/cgroup/net_cls,net_prio cgroup rw,nosuid,nodev,noexec,relatime,net_cls,net_prio 0 0
cgroup /sys/fs/cgroup/blkio cgroup rw,nosuid,nodev,noexec,relatime,blkio 0 0
cgroup /sys/fs/cgroup/cpuset cgroup rw,nosuid,nodev,noexec,relatime,cpuset 0 0
cgroup /sys/fs/cgroup/pids cgroup rw,nosuid,nodev,noexec,relatime,pids 0 0
cgroup /sys/fs/cgroup/hugetlb cgroup rw,nosuid,nodev,noexec,relatime,hugetlb 0 0
cgroup /sys/fs/cgroup/rdma cgroup rw,nosuid,nodev,noexec,relatime,rdma 0 0
cgroup /sys/fs/cgroup/devices cgroup rw,nosuid,nodev,noexec,relatime,devices 0 0
cgroup /sys/fs/cgroup/memory cgroup rw,nosuid,nodev,noexec,relatime,memory 0 0
cgroup /sys/fs/cgroup/cpu,cpuacct cgroup rw,nosuid,nodev,noexec,relatime,cpu,cpuacct 0 0
cgroup /sys/fs/cgroup/misc cgroup rw,nosuid,nodev,noexec,relatime,misc 0 0
cgroup /sys/fs/cgroup/perf_event cgroup rw,nosuid,nodev,noexec,relatime,perf_event 0 0
cgroup /sys/fs/cgroup/freezer cgroup rw,nosuid,nodev,noexec,relatime,freezer 0 0
/dev/nvme0n1p2 / ext4 rw,relatime 0 0
rpc_pipefs /var/lib/nfs/rpc_pipefs rpc_pipefs rw,relatime 0 0
systemd-1 /proc/sys/fs/binfmt_misc autofs rw,relatime,fd=30,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=23919 0 0
mqueue /dev/mqueue mqueue rw,nosuid,nodev,noexec,relatime 0 0
hugetlbfs /dev/hugepages hugetlbfs rw,relatime,pagesize=2M 0 0
debugfs /sys/kernel/debug debugfs rw,nosuid,nodev,noexec,relatime 0 0
tracefs /sys/kernel/tracing tracefs rw,nosuid,nodev,noexec,relatime 0 0
binfmt_misc /proc/sys/fs/binfmt_misc binfmt_misc rw,nosuid,nodev,noexec,relatime 0 0
configfs /sys/kernel/config configfs rw,nosuid,nodev,noexec,relatime 0 0
fusectl /sys/fs/fuse/connections fusectl rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /tmp tmpfs rw,nosuid,nodev,size=7819764k,nr_inodes=409600 0 0
/dev/nvme0n1p1 /efi vfat rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,utf8,errors=remount-ro 0 0
tmpfs /run/user/1000 tmpfs rw,nosuid,nodev,relatime,size=1563952k,nr_inodes=390988,mode=700,uid=1000,gid=1001 0 0
gvfsd-fuse /run/user/1000/gvfs fuse.gvfsd-fuse rw,nosuid,nodev,relatime,user_id=1000,group_id=1001 0 0"#[..]).unwrap().1;

    assert_eq!(mounts[0], ("proc", "/proc"));
    assert_eq!(mounts[1], ("sysfs", "/sys"));
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
