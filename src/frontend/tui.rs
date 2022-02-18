use crate::{
    disks::{self, ALLOWED_FS_TYPE},
    install::{self, umount_all},
    network::{self, Mirror, VariantEntry},
};
use anyhow::Result;
use cursive::{
    event::Event,
    views::{
        Dialog, DummyView, EditView, LinearLayout, ListView, NamedView, Panel, ProgressBar,
        RadioGroup, ResizedView, ScrollView, SelectView, TextContent, TextView,
    },
};
use cursive::{traits::*, utils::Counter};
use cursive::{view::SizeConstraint, views::Button};
use cursive::{Cursive, View};
use cursive_async_view::AsyncView;
use cursive_table_view::{TableView, TableViewItem};
use number_prefix::NumberPrefix;
use std::process::Command;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc, thread};
use std::{env, fs, io::Read, path::PathBuf};
use tempfile::TempDir;

use super::{begin_install, InstallConfig};

const LAST_USER_CONFIG_FILE: &str = "/tmp/deploykit-config.json";
const SAVE_USER_CONFIG_FILE: &str = "/root/deploykit-config.json";
macro_rules! SURE_FS_TYPE_INFO {
    () => {
        "The current partition format is {}, do you want to use this partition format? We recommend that you use ext4 as the partition format, as it is generally trouble-free."
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum VariantColumn {
    Name,
    Date,
    Size,
}

impl TableViewItem<VariantColumn> for network::VariantEntry {
    fn to_column(&self, column: VariantColumn) -> String {
        match column {
            VariantColumn::Name => self.name.clone(),
            VariantColumn::Date => self.date.clone(),
            VariantColumn::Size => human_size(self.size),
        }
    }
    fn cmp(&self, other: &Self, column: VariantColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            VariantColumn::Name => self.name.cmp(&other.name),
            VariantColumn::Date => self.date.cmp(&other.date),
            VariantColumn::Size => self.size.cmp(&other.size),
        }
    }
}
macro_rules! SUMMARY_TEXT {
    () => {
        "The following actions will be performed:\n- {} will be erased and formatted as {}.\n- AOSC OS {} variant will be installed using {}.\n- User {} will be created.\n- AOSC OS will use the {} locale.\n- Your timezone will be set to {}, and will use {} as system time."
    };
}

const WELCOME_TEXT: &str = r#"Welcome to the AOSC OS installer!

In the following pages, the installer will guide you through selection of distribution, download sources, partitioning, and other system configurations. The installation process should only take a few minutes, but will require more time on slower hardware."#;
const VARIANT_TEXT: &str = "Shown below is a list of available distributions for your computer.";
const ENTER_USER_PASSWORD_TEXT: &str = r#"Please enter and confirm your desired username and password. Please note that your username must start with a lower-cased alphabetical letter (a-z), and contain only lower-cased letters a-z, numbers 0-0, and dash ("-").
"#;
const ENTER_HOSTNAME_TEXT: &str = r#"Now, please input your desired hostname. A hostname may only consist letters a-z, numbers 0-9, and dash ("-")"#;
const ENTER_TIMEZONE_TEXT: &str = r#"Finally, please select your locale, timezone, and your clock preferences. Your locale setting will affect your installation's display language. UTC system time is the default setting for Linux systems, but may result in time discrepancy with your other operating systems, such as Windows. If you wish to prevent this from happening, please select local time as system time."#;
const BENCHMARK_TEXT: &str = "DeployKit will now test all mirrors for download speed, and rank them from the fastest (top) to the slowest (bottom). This may take a few minutes.";
const FINISHED_TEXT: &str = r#"AOSC OS has been successfully installed on your device.

You may reboot to your installed system by choosing "Reboot," or return to LiveKit by selecting "Exit to LiveKit.""#;

macro_rules! fill_in_all_the_fields {
    ($s:ident) => {
        show_msg($s, "Please fill in all the fields.");
        return;
    };
}

macro_rules! show_fetch_progress {
    ($siv:ident, $m:tt, $e:tt, $f:block) => {{
        $siv.pop_layer();
        $siv.add_layer(
            Dialog::around(TextView::new(format!("{}\nThis can take a while ...", $m)))
                .title("Progress"),
        );
        $siv.refresh();
        let ret = { $f };
        if ret.is_err() {
            show_error($siv, $e);
            return;
        }
        $siv.pop_layer();
        ret.unwrap()
    }};
    ($siv:ident, $m:tt, $f:block) => {{
        $siv.pop_layer();
        $siv.add_layer(
            Dialog::around(TextView::new(format!("{}\nThis can take a while ...", $m)))
                .title("Progress"),
        );
        // $siv.refresh();
        let ret = { $f };
        $siv.pop_layer();
        ret
    }};
}

fn show_error(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::around(TextView::new(msg))
            .title("Error")
            .button("Exit", |s| s.quit())
            .padding_lrtb(2, 2, 1, 1),
    );
}

fn show_msg(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::around(TextView::new(msg))
            .title("Message")
            .button("OK", |s| {
                s.pop_layer();
            })
            .padding_lrtb(2, 2, 1, 1),
    );
}

fn show_blocking_message(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::around(TextView::new(msg))
            .title("Message")
            .padding_lrtb(2, 2, 1, 1),
    );
}

fn partition_button() -> (&'static str, &'static dyn Fn(&mut Cursive, InstallConfig)) {
    if env::var("DISPLAY").is_ok() {
        return ("Open GParted", &|s, _| {
            show_blocking_message(s, "Waiting for GParted Partitioning Program to finish ...");
            let cb_sink = s.cb_sink().clone();
            thread::spawn(move || {
                Command::new("gparted").output().ok();
                cb_sink
                    .send(Box::new(|s| {
                        let new_parts = disks::list_partitions();
                        let (disk_list, disk_view) = make_partition_list(new_parts);
                        s.set_user_data(disk_list);
                        s.call_on_name("part_list", |view: &mut NamedView<LinearLayout>| {
                            *view = disk_view;
                        });
                        s.pop_layer();
                    }))
                    .unwrap();
            });
        });
    }
    ("Open Shell", &|s, config| {
        s.set_user_data(config);
        let dump = s.dump();
        s.quit();
        s.set_user_data(dump);
    })
}

#[inline]
fn human_size(size: u64) -> String {
    match NumberPrefix::binary(size as f64) {
        NumberPrefix::Standalone(bytes) => format!("{} B", bytes),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}B", n, prefix),
    }
}

fn make_partition_list(
    partitions: Vec<disks::Partition>,
) -> (RadioGroup<disks::Partition>, NamedView<LinearLayout>) {
    let mut disk_view = LinearLayout::vertical();
    let mut disk_list = RadioGroup::new();
    for part in &partitions {
        let path_name;
        if let Some(path) = &part.path {
            path_name = path.to_string_lossy().to_string();
        } else {
            path_name = "?".to_owned();
        }
        let radio = disk_list.button(
            part.clone(),
            format!(
                "{} ({}, {})",
                path_name,
                part.fs_type.as_ref().unwrap_or(&"?".to_owned()),
                human_size(part.size)
            ),
        );
        disk_view.add_child(radio);
    }
    if partitions.is_empty() {
        let dummy_partition = disks::Partition {
            path: None,
            parent_path: None,
            fs_type: None,
            size: 0,
        };
        disk_view.add_child(disk_list.button(dummy_partition, "No partition selected"));
    }

    (disk_list, disk_view.with_name("part_list"))
}

fn make_locale_list(locales: Vec<String>) -> SelectView {
    let locale_view = SelectView::new()
        .popup()
        .autojump()
        .with_all_str(locales.iter());

    locale_view
}

fn make_continent_list(zoneinfo: Vec<(String, Vec<String>)>) -> SelectView {
    SelectView::new().popup().autojump().with_all_str(
        zoneinfo
            .into_iter()
            .map(|(con, _)| con)
            .collect::<Vec<String>>(),
    )
}

fn wrap_in_dialog<V: View, S: Into<String>>(inner: V, title: S, width: Option<usize>) -> Dialog {
    Dialog::around(ResizedView::new(
        SizeConstraint::AtMost(width.unwrap_or(64)),
        SizeConstraint::Free,
        ScrollView::new(inner),
    ))
    .padding_lrtb(2, 2, 1, 1)
    .title(title)
}

fn build_variant_list(
    mirrors: Vec<Mirror>,
    variants: Vec<VariantEntry>,
    config: InstallConfig,
) -> Dialog {
    let mut config_view = LinearLayout::vertical();

    let variant_view = TableView::<network::VariantEntry, VariantColumn>::new()
        .column(VariantColumn::Name, "Available Distribution", |c| {
            c.width(60)
        })
        .column(VariantColumn::Date, "Last Updated", |c| c.width(20))
        .column(VariantColumn::Size, "Download Size", |c| c.width(20))
        .items(variants.clone())
        .on_submit(move |siv, _row, index| {
            let mut config = config.clone();
            config.variant = Some(Arc::new(variants.get(index).unwrap().clone()));
            select_mirrors(siv, mirrors.clone(), config);
        })
        .min_width(106)
        .min_height(30);
    let variant_view = Panel::new(variant_view).title("Variant");
    config_view.add_child(TextView::new(VARIANT_TEXT));
    config_view.add_child(variant_view);
    config_view.add_child(DummyView {});

    wrap_in_dialog(config_view, "AOSC OS Installation", Some(128)).button("Exit", |s| s.quit())
}

fn select_variant(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let loader = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let manifest = network::fetch_recipe().map_err(|e| e.to_string())?;
            let mirrors = network::fetch_mirrors(&manifest);
            let variants = network::find_variant_candidates(manifest).map_err(|e| e.to_string())?;
            Ok((mirrors, variants))
        },
        move |(mirrors, variants)| build_variant_list(mirrors, variants, config.clone()),
    );

    siv.add_layer(loader);
}

fn select_mirrors(siv: &mut Cursive, mirrors: Vec<Mirror>, config: InstallConfig) {
    siv.pop_layer();
    let (config_view, repo_list) = select_mirror_view_base(&mirrors);
    siv.add_layer(select_mirrors_view(config_view, config, repo_list, mirrors));
}

fn select_mirror_view_base(mirrors: &[Mirror]) -> (LinearLayout, RadioGroup<Mirror>) {
    let mut config_view = LinearLayout::vertical();
    let mut repo_list = RadioGroup::new();
    let mirror_list = &*mirrors;
    let mut repo_view = LinearLayout::vertical()
        .child(TextView::new(
            "Please select a mirror from which you would like to download AOSC OS and the extra components you specified. Generally, a mirror closest to you geographically would be the best bet for download speeds.",
        ))
        .child(DummyView {});
    for mirror in mirror_list {
        let radio = repo_list.button(mirror.clone(), format!("{} ({})", mirror.name, mirror.loc));
        repo_view.add_child(radio);
    }
    let repo_view = Panel::new(repo_view).title("Repositories");
    config_view.add_child(repo_view);
    config_view.add_child(DummyView {});

    (config_view, repo_list)
}

fn select_mirrors_view(
    config_view: LinearLayout,
    config: InstallConfig,
    repo_list: RadioGroup<Mirror>,
    mirrors: Vec<Mirror>,
) -> Dialog {
    let config_clone = config.clone();
    let config_clone_2 = config.clone();
    wrap_in_dialog(config_view, "AOSC OS Installation", None)
        .button("Continue", move |s| {
            let mut config = config.clone();
            let mirror = repo_list.selection();
            config.mirror = Some(Arc::new(Rc::as_ref(&mirror).clone()));
            if config.partition.is_some() {
                select_user_password(s, config);
            } else {
                select_partition(s, config);
            }
        })
        .button("Benchmark Mirrors", move |s| {
            let config_clone_2 = config_clone.clone();
            let config_clone_3 = config_clone.clone();
            let mirrors_clone = mirrors.clone();
            let mirrors_clone_2 = mirrors.clone();
            s.pop_layer();
            s.add_layer(
                Dialog::around(TextView::new(BENCHMARK_TEXT).max_width(80))
                    .title("Message")
                    .button("OK", move |s| {
                        let config_clone_3 = config_clone_2.clone();
                        let mirrors_clone_2 = mirrors_clone.clone();
                        let loader = AsyncView::new_with_bg_creator(
                            s,
                            move || {
                                let new_mirrors = network::speedtest_mirrors(mirrors_clone_2);
                                Ok(new_mirrors)
                            },
                            move |mirrors| {
                                let (config_view, repo_list) = select_mirror_view_base(&mirrors);
                                select_mirrors_view(
                                    config_view,
                                    config_clone_3.clone(),
                                    repo_list,
                                    mirrors,
                                )
                            },
                        );
                        s.pop_layer();
                        s.add_layer(loader);
                    })
                    .button("Cancel", move |s| {
                        let mirrors_clone_3 = mirrors_clone_2.clone();
                        let config_clone_4 = config_clone_3.clone();
                        s.pop_layer();
                        select_mirrors(s, mirrors_clone_3, config_clone_4);
                    })
                    .padding_lrtb(2, 2, 1, 1),
            );
        })
        .button("Back", move |s| {
            s.pop_layer();
            select_variant(s, config_clone_2.clone());
        })
        .button("Exit", |s| s.quit())
}

fn select_partition(siv: &mut Cursive, config: InstallConfig) {
    let partitions = show_fetch_progress!(siv, "Probing disks ...", { disks::list_partitions() });
    let (disk_list, disk_view) = make_partition_list(partitions);
    siv.set_user_data(disk_list);
    let dest_view = LinearLayout::vertical()
    .child(TextView::new(
        "Please select a partition to which you would like to install AOSC OS onto. If you would like to make changes to your partitions, please click on \"Open GParted.\"",
    ))
    .child(DummyView {})
    .child(disk_view);
    let config_view = LinearLayout::vertical()
        .child(Panel::new(dest_view).title("Destination"))
        .child(DummyView {});
    let (btn_label, btn_cb) = partition_button();
    let config_copy = config.clone();
    let config_copy_2 = config.clone();
    let config_clone_3 = config.clone();
    siv.add_layer(
        wrap_in_dialog(config_view, "AOSC OS Installation", None)
        .button("Continue", move |s| {
            let disk_list = s.user_data::<RadioGroup<disks::Partition>>();
            let required_size = config_clone_3.variant.as_ref().unwrap().install_size;
            if let Some(disk_list) = disk_list {
                let disk_list = disk_list.clone();
                let current_partition = if cfg!(debug_assertions) {
                    // prevent developer/tester accidentally delete their partitions
                    Rc::new(disks::Partition {
                        fs_type: Some("xfs".to_string()),
                        path: Some(PathBuf::from("/dev/loop0p1")),
                        parent_path: Some(PathBuf::from("/dev/loop0")),
                        size: required_size,
                    })
                } else {
                    disk_list.selection()
                };
                if current_partition.parent_path.is_none() && current_partition.size == 0 {
                    show_msg(s, "Please specify a partition.");
                    // s.refresh();
                    return;
                }
                if current_partition.size < required_size {
                    show_msg(
                        s,
                        &format!(
                            "The selected partition is not enough to install this tarball!\nCurrent disk size: {:.3}GiB\nDisk size required: {:.3}GiB", 
                            current_partition.size as f32 / 1024.0 / 1024.0 / 1024.0, // 1024 * 1024 * 1024 = 11073741824
                            required_size as f32 / 1024.0 / 1024.0 / 1024.0
                        ));
                    return;
                }
                let mut config = config.clone();
                let config_copy = config.clone();
                let config_copy_2 = config.clone();
                let config_copy_3 = config.clone();
                let fs_type = current_partition.fs_type.as_ref();
                let current_partition_clone = current_partition.clone();
                if fs_type != Some(&"ext4".to_string()) && ALLOWED_FS_TYPE.contains(&fs_type.unwrap().as_str()) {
                    let view = wrap_in_dialog(LinearLayout::vertical()
                    .child(TextView::new(format!(SURE_FS_TYPE_INFO!(), fs_type.unwrap()))), "AOSC OS Installer", None)
                    .button("Yes", move |s| {
                        let new_part = disks::fill_fs_type(current_partition_clone.as_ref(), false);
                        let mut config_clone = config_copy.clone();
                        config_clone.partition = Some(Arc::new(new_part));
                        s.pop_layer();
                        if config.user.is_some() {
                            is_use_last_config(s, config_clone);
                        } else {
                            select_user_password(s, config_clone);
                        }
                    })
                    .button("Use Ext4", move |s| {
                        let new_part = disks::fill_fs_type(current_partition.as_ref(), true);
                        let mut config_clone = config_copy_2.clone();
                        config_clone.partition = Some(Arc::new(new_part));
                        s.pop_layer();
                        if config_clone.user.is_some() {
                            is_use_last_config(s, config_clone);
                        } else {
                            select_user_password(s, config_clone);
                        }
                    })
                    .button("Cancel", move |s| {
                        s.pop_layer();
                        btn_cb(s, config_copy_3.clone());
                    });
                    s.add_layer(view);
                } else {
                    let new_part = disks::fill_fs_type(current_partition_clone.as_ref(), false);
                    config.partition = Some(Arc::new(new_part));
                    if config.user.is_some() {
                        is_use_last_config(s, config);
                    } else {
                        select_user_password(s, config);
                    }
                }
            }
        })
        .button(btn_label, move |s| {
            btn_cb(s, config_copy.clone());
        })
        .button("Back", move |s| {
            s.pop_layer();
            select_variant(s, config_copy_2.clone());
        })
        .button("Exit", |s| s.quit())
    );
}

fn select_user_password(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let password = Rc::new(RefCell::new(String::new()));
    let password_copy = Rc::clone(&password);
    let password_confirm = Rc::new(RefCell::new(String::new()));
    let password_confirm_copy = Rc::clone(&password_confirm);
    let name = Rc::new(RefCell::new(String::new()));
    let name_copy = Rc::clone(&name);

    let user_password_textview = TextView::new(ENTER_USER_PASSWORD_TEXT).max_width(80);
    let user_password_view = ListView::new()
        .child(
            "Username",
            EditView::new()
                .on_edit_mut(move |_, c, _| {
                    name_copy.replace(c.to_owned());
                })
                .min_width(20)
                .with_name("user"),
        )
        .child(
            "Password",
            EditView::new()
                .secret()
                .on_edit_mut(move |_, c, _| {
                    password_copy.replace(c.to_owned());
                })
                .min_width(20)
                .with_name("pwd"),
        )
        .child(
            "Confirm Password",
            EditView::new()
                .secret()
                .on_edit_mut(move |_, c, _| {
                    password_confirm_copy.replace(c.to_owned());
                })
                .min_width(20)
                .with_name("pwd2"),
        );
    let config_clone = config.clone();
    let user_password_dialog = wrap_in_dialog(
        LinearLayout::vertical()
            .child(user_password_textview)
            .child(DummyView {})
            .child(user_password_view),
        "AOSC OS Installer",
        None,
    )
    .button("Continue", move |s| {
        let password = password.as_ref().to_owned().into_inner();
        let password_confirm = password_confirm.as_ref().to_owned().into_inner();
        let name = name.as_ref().to_owned().into_inner();
        if password.is_empty() || password_confirm.is_empty() || name.is_empty() {
            fill_in_all_the_fields!(s);
        }
        if password != password_confirm {
            show_msg(s, "Password and confirm password do not match.");
            return;
        }
        let mut config = config.clone();
        config.password = Some(Arc::new(password));
        config.user = Some(Arc::new(name));
        select_hostname(s, config);
    })
    .button("Back", move |s| {
        s.pop_layer();
        select_partition(s, config_clone.clone());
    })
    .button("Exit", |s| s.quit());

    siv.add_layer(user_password_dialog);
}

fn select_hostname(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let hostname = Rc::new(RefCell::new(String::new()));
    let hostname_copy = Rc::clone(&hostname);
    let hostname_textview = TextView::new(ENTER_HOSTNAME_TEXT);
    let hostname_view = ListView::new()
        .child(
            "Hostname",
            EditView::new()
                .on_edit_mut(move |_, c, _| {
                    hostname_copy.replace(c.to_owned());
                })
                .min_width(20)
                .with_name("hostname"),
        )
        .delimiter();
    let config_clone = config.clone();
    let hostname_dialog = wrap_in_dialog(
        LinearLayout::vertical()
            .child(hostname_textview)
            .child(DummyView {})
            .child(hostname_view),
        "AOSC OS Installer",
        None,
    )
    .button("Continue", move |s| {
        let hostname = hostname.as_ref().to_owned().into_inner();
        if hostname.is_empty() {
            fill_in_all_the_fields!(s);
        }
        let mut config = config.clone();
        config.hostname = Some(hostname);
        select_timezone(s, config);
    })
    .button("Back", move |s| {
        s.pop_layer();
        select_user_password(s, config_clone.clone());
    })
    .button("Exit", |s| s.quit());

    siv.add_layer(hostname_dialog);
}

fn select_timezone(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    // locale default is C.UTF-8
    let locale = Rc::new(RefCell::new(String::from("C.UTF-8")));
    let locale_copy = Rc::clone(&locale);
    let continent = Rc::new(RefCell::new(String::new()));
    let continent_copy = Rc::clone(&continent);
    let city = Rc::new(RefCell::new(String::new()));
    let city_copy = Rc::clone(&city);
    // RTC/UTC default is UTC
    let tc = Rc::new(RefCell::new(String::from("UTC")));
    let tc_copy = Rc::clone(&tc);
    let locales = install::get_locale_list().unwrap();
    let timezone_textview = TextView::new(ENTER_TIMEZONE_TEXT);
    let mut timezone_selected_status = TextView::new("N/A");
    let status_text = Arc::new(timezone_selected_status.get_shared_content());
    let timezone_view = ListView::new()
        .child(
            "Timezone",
            Button::new("Set Timezone", move |s| {
                let zoneinfo = install::get_zoneinfo_list().unwrap();
                let city_clone = Rc::clone(&city_copy);
                let continent_copy_copy = Rc::clone(&continent_copy);
                s.add_layer(set_timezone(
                    zoneinfo,
                    city_clone,
                    continent_copy_copy,
                    status_text.clone(),
                ))
            }),
        )
        .child("Selected Timezone", timezone_selected_status.center())
        .child(
            "Locale",
            make_locale_list(locales)
                .on_submit(move |_, c: &String| {
                    locale_copy.replace(c.to_owned());
                })
                .min_width(20),
        )
        .child(
            "RTC Timezone",
            SelectView::new()
                .autojump()
                .popup()
                .with_all_str(vec!["UTC (Recommended)", "Local time (like Windows)"])
                .on_submit(move |_, c: &str| {
                    let selected = match c {
                        "UTC (Recommended)" => "UTC",
                        "Local time (like Windows)" => "RTC",
                        _ => unreachable!(),
                    };
                    tc_copy.replace(selected.to_string());
                })
                .min_width(20),
        );
    let config_clone = config.clone();
    let timezone_dialog = wrap_in_dialog(
        LinearLayout::vertical()
            .child(timezone_textview)
            .child(DummyView {})
            .child(timezone_view),
        "AOSC OS Installer",
        None,
    )
    .button("Continue", move |s| {
        let locale = locale.as_ref().to_owned().into_inner();
        let continent = continent.as_ref().to_owned().into_inner();
        let city = city.as_ref().to_owned().into_inner();
        let tc = tc.as_ref().to_owned().into_inner();
        if locale.is_empty() || continent.is_empty() || city.is_empty() || tc.is_empty() {
            fill_in_all_the_fields!(s);
        }
        let mut config = config.clone();
        config.locale = Some(Arc::new(locale));
        config.continent = Some(Arc::new(continent));
        config.city = Some(Arc::new(city));
        config.tc = Some(Arc::new(tc));
        show_summary(s, config);
    })
    .button("Back", move |s| {
        s.pop_layer();
        select_hostname(s, config_clone.clone());
    })
    .button("Exit", |s| s.quit());

    siv.add_layer(timezone_dialog);
}

fn set_timezone(
    zoneinfo: Vec<(String, Vec<String>)>,
    city_clone: Rc<RefCell<String>>,
    continent_copy_copy: Rc<RefCell<String>>,
    status_text: Arc<TextContent>,
) -> Dialog {
    wrap_in_dialog(
        make_continent_list(zoneinfo.clone()).on_submit(move |s, c: &String| {
            let zoneinfo_clone = zoneinfo.clone();
            let index = zoneinfo.iter().position(|(x, _)| x == c).unwrap();
            let citys = &zoneinfo[index].1;
            let city_clone_clone = Rc::clone(&city_clone);
            let city_clone_3 = Rc::clone(&city_clone_clone);
            let continent_clone_3 = Rc::clone(&continent_copy_copy);
            let status_text_copy = status_text.clone();
            status_text_copy.set_content("");
            status_text_copy.append(format!("{}/", c));
            let status_text_copy_copy = status_text_copy.clone();
            continent_copy_copy.replace(c.to_string());
            s.pop_layer();
            s.add_layer(
                wrap_in_dialog(
                    LinearLayout::vertical().child(
                        SelectView::new()
                            .autojump()
                            .popup()
                            .with_all_str(citys.iter())
                            .on_submit(move |_, c: &String| {
                                city_clone_clone.replace(c.to_string());
                                status_text_copy.append(c.to_string());
                            })
                            .min_width(20),
                    ),
                    "Set city",
                    None,
                )
                .button("Ok", |s| {
                    s.cb_sink()
                        .send(Box::new(|s| {
                            s.pop_layer();
                        }))
                        .unwrap()
                })
                .button("Cancel", move |s| {
                    s.pop_layer();
                    s.add_layer(set_timezone(
                        zoneinfo_clone.clone(),
                        Rc::clone(&city_clone_3),
                        Rc::clone(&continent_clone_3),
                        status_text_copy_copy.clone(),
                    ));
                }),
            )
        }),
        "Set continent",
        None,
    )
}

fn is_use_last_config(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let config_copy = config.clone();
    siv.add_layer(
        wrap_in_dialog(
            TextView::new("Using the last configuration?"),
            "AOSC OS Installer",
            None,
        )
        .button("Yes", move |s| show_summary(s, config_copy.clone()))
        .button("No", move |s| {
            fs::remove_file(LAST_USER_CONFIG_FILE).ok();
            let new_config = InstallConfig {
                variant: None,
                partition: config.clone().partition,
                mirror: None,
                user: None,
                password: None,
                hostname: None,
                locale: None,
                continent: None,
                city: None,
                tc: None,
            };
            select_variant(s, new_config);
        })
        .button("Exit", |s| s.quit()),
    );
}

fn show_summary(siv: &mut Cursive, config: InstallConfig) {
    let mut path = String::new();
    let mut fs = String::new();
    let config_copy = config.clone();
    let config_copy_2 = config.clone();
    if let Some(partition) = config.partition {
        if let Some(partition) = &partition.path {
            path = partition.to_string_lossy().to_string();
        }
        if let Some(fs_type) = &partition.fs_type {
            fs = fs_type.clone();
        }
    }
    siv.add_layer(
        wrap_in_dialog(
            TextView::new(format!(
                SUMMARY_TEXT!(),
                path,
                fs,
                config.variant.unwrap().name,
                config.mirror.unwrap().name,
                config.user.unwrap(),
                config.locale.unwrap(),
                format_args!("{}/{}", config.continent.unwrap(), config.city.unwrap()),
                config.tc.unwrap()
            )),
            "Confirmation",
            None,
        )
        .button("Install", move |s| {
            s.pop_layer();
            start_install(s, config_copy.clone());
        })
        .button("Save Config", move |s| {
            if let Err(e) = save_user_config_to_file(config_copy_2.clone(), SAVE_USER_CONFIG_FILE) {
                show_error(s, &e.to_string())
            } else {
                show_msg(
                    s,
                    &format!("Success saved, path: {}!", SAVE_USER_CONFIG_FILE),
                )
            }
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn start_install(siv: &mut Cursive, config: InstallConfig) {
    siv.clear_global_callbacks(Event::Exit);
    siv.clear_global_callbacks(Event::CtrlChar('c'));
    ctrlc::set_handler(|| {}).expect("Error setting SIGINT handler.");
    save_user_config_to_file(config.clone(), LAST_USER_CONFIG_FILE).ok();
    siv.pop_layer();
    let counter = Counter::new(0);
    let counter_clone = counter.clone();
    let mut status_message = TextView::new("");
    let status_text = Arc::new(status_message.get_shared_content());
    siv.add_layer(wrap_in_dialog(
        LinearLayout::vertical()
            .child(TextView::new(
                "Please wait while the installation takes place. This may take minutes or in extreme cases, hours, depending on your device's performance.",
            ))
            .child(DummyView {})
            .child(ProgressBar::new().max(100).with_value(counter))
            .child(status_message),
        "Installing",
        None,
    ));

    let (tx, rx) = std::sync::mpsc::channel();
    siv.set_autorefresh(true);
    let cb_sink = siv.cb_sink().clone();
    let tempdir = TempDir::new()
        .expect("Unable to create temporary directory")
        .into_path();
    let tempdir_copy = tempdir.clone();
    let root_fd = install::get_dir_fd(PathBuf::from("/"));
    let install_thread = thread::spawn(move || begin_install(tx, config, tempdir_copy));
    thread::spawn(move || loop {
        if let Ok(progress) = rx.recv() {
            match progress {
                super::InstallProgress::Pending(msg, pct) => {
                    counter_clone.set(pct);
                    status_text.set_content(msg);
                }
                super::InstallProgress::Finished => {
                    cb_sink.send(Box::new(show_finished)).unwrap();
                    return;
                }
            }
        } else {
            let err = install_thread.join().unwrap().unwrap_err();
            if let Ok(root_fd) = root_fd {
                umount_all(&tempdir, root_fd);
            }
            cb_sink
                .send(Box::new(move |s| {
                    show_error(s, &err.to_string());
                }))
                .unwrap();
            return;
        }
    });
}

fn save_user_config_to_file(config: InstallConfig, path: &str) -> Result<()> {
    let mut config_copy = config;
    config_copy.partition = None;
    let file_str = serde_json::to_string(&config_copy)?;
    fs::File::create(LAST_USER_CONFIG_FILE)?;
    fs::write(path, file_str)?;

    Ok(())
}

fn read_user_config_on_file() -> Result<InstallConfig> {
    let mut file = fs::File::open(LAST_USER_CONFIG_FILE)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(serde_json::from_slice(&buffer)?)
}

fn show_finished(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        wrap_in_dialog(TextView::new(FINISHED_TEXT), "All Done", None)
            .button("Reboot", |s| {
                install::sync_and_reboot().ok();
                s.quit();
            })
            .button("Exit to LiveKit", |s| s.quit()),
    );
}

pub fn tui_main() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(TextView::new(WELCOME_TEXT))
            .title("Welcome")
            .button("Let's Go", |s| {
                if let Ok(config) = read_user_config_on_file() {
                    select_partition(s, config);
                } else {
                    let config = InstallConfig {
                        variant: None,
                        partition: None,
                        mirror: None,
                        user: None,
                        password: None,
                        hostname: None,
                        locale: None,
                        continent: None,
                        city: None,
                        tc: None,
                    };
                    select_variant(s, config);
                }
            })
            .padding_lrtb(2, 2, 1, 1)
            .max_width(80),
    );
    siv.run();
    loop {
        let dump = siv.take_user_data::<cursive::Dump>();
        if let Some(dump) = dump {
            drop(siv);
            println!("You can use tools like cfdisk or gdisk to modify your partitions.\nExit the shell to return to the installer.");
            std::process::Command::new("bash")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            siv = cursive::default();
            siv.restore(dump);
            let config = siv.take_user_data::<InstallConfig>();
            if let Some(config) = config {
                select_partition(&mut siv, config);
                siv.run();
            }
        } else {
            break;
        }
    }
}
