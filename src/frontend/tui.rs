use crate::{
    disks, install,
    network::{self, Mirror, VariantEntry},
};
use cursive::view::SizeConstraint;
use cursive::views::{
    Dialog, DummyView, EditView, LinearLayout, ListView, NamedView, Panel, ProgressBar, RadioGroup,
    ResizedView, ScrollView, SelectView, TextView,
};
use cursive::{traits::*, utils::Counter};
use cursive::{Cursive, View};
use cursive_async_view::AsyncView;
use cursive_table_view::{TableView, TableViewItem};
use number_prefix::NumberPrefix;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc, thread};

use super::{begin_install, InstallConfig};

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
        "The following actions will be performed:\n- {} will be erased and formatted as {}.\n- AOSC OS {} variant will be installed using {} mirror server.\n- User {} will be created."
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
                cb_sink.send(Box::new(|s| {
                    let new_parts = disks::list_partitions();
                    let (disk_list, disk_view) = make_partition_list(new_parts);
                    s.set_user_data(disk_list);
                    s.call_on_name("part_list", |view: &mut NamedView<LinearLayout>| {
                        *view = disk_view;
                    });
                    s.pop_layer();
                })).unwrap();
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
        .column(VariantColumn::Name, "Name", |c| c.width(60))
        .column(VariantColumn::Date, "Date", |c| c.width(20))
        .column(VariantColumn::Size, "Size", |c| c.width(20))
        .items(variants.clone())
        .on_submit(move |siv, _row, index| {
            let mut config = config.clone();
            config.variant = Some(Arc::new(variants.get(index).unwrap().clone()));
            select_mirrors(siv, mirrors.clone(), config);
        })
        .min_width(106)
        .min_height(30);
    let variant_view = Panel::new(variant_view).title("Variant");
    config_view.add_child(variant_view);
    config_view.add_child(DummyView {});

    wrap_in_dialog(config_view, "AOSC OS Installation", Some(128))
}

fn select_variant(siv: &mut Cursive, config: InstallConfig) {
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
    let mut config_view = LinearLayout::vertical();

    let mut repo_list = RadioGroup::new();
    let mirror_list = mirrors;
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
    siv.add_layer(
        wrap_in_dialog(config_view, "AOSC OS Installation", None).button("Continue", move |s| {
            let mut config = config.clone();
            let mirror = repo_list.selection();
            config.mirror = Some(Arc::new(Rc::as_ref(&mirror).clone()));
            select_partition(s, config);
        }),
    );
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
    siv.add_layer(
        wrap_in_dialog(config_view, "AOSC OS Installation", None)
            .button(btn_label, move |s| {
                btn_cb(s, config_copy.clone());
            })
            .button("Continue", move |s| {
                let disk_list = s.user_data::<RadioGroup<disks::Partition>>();
                if let Some(disk_list) = disk_list {
                    let disk_list = disk_list.clone();
                    let current_partition = if cfg!(debug_assertions) {
                        // prevent developer/tester accidentally delete their partitions
                        Rc::new(disks::Partition {
                            fs_type: None,
                            path: Some(PathBuf::from("/dev/loop0p1")),
                            parent_path: Some(PathBuf::from("/dev/loop0")),
                            size: 3145728,
                        })
                    } else {
                        disk_list.selection()
                    };
                    if current_partition.parent_path.is_none() && current_partition.size == 0 {
                        show_msg(s, "Please specify a partition.");
                        // s.refresh();
                        return;
                    }
                    let mut config = config.clone();
                    let new_part = disks::fill_fs_type(current_partition.as_ref());
                    config.partition = Some(Arc::new(new_part));
                    select_user(s, config);
                }
            }),
    );
}

fn select_user(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let locales = install::get_locale_list().unwrap();
    let password = Rc::new(RefCell::new(String::new()));
    let password_copy = Rc::clone(&password);
    let password_confirm = Rc::new(RefCell::new(String::new()));
    let password_confirm_copy = Rc::clone(&password_confirm);
    let name = Rc::new(RefCell::new(String::new()));
    let name_copy = Rc::clone(&name);
    let hostname = Rc::new(RefCell::new(String::new()));
    let hostname_copy = Rc::clone(&hostname);
    let locale = Rc::new(RefCell::new(String::new()));
    let locale_copy = Rc::clone(&locale);

    let config_view = ListView::new()
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
        )
        .delimiter()
        .child(
            "Hostname",
            EditView::new()
                .on_edit_mut(move |_, c, _| {
                    hostname_copy.replace(c.to_owned());
                })
                .min_width(20)
                .with_name("hostname"),
        )
        .child(
            "Locale",
            make_locale_list(locales)
                .on_select(move |_, c| {
                    locale_copy.replace(c.to_owned());
                })
                .min_width(20),
        );
    siv.add_layer(
        wrap_in_dialog(config_view, "AOSC OS Installation", None).button("Continue", move |s| {
            let password = password.as_ref().to_owned().into_inner();
            let password_confirm = password_confirm.as_ref().to_owned().into_inner();
            let name = name.as_ref().to_owned().into_inner();
            let hostname = hostname.as_ref().to_owned().into_inner();
            let locale = locale.as_ref().to_owned().into_inner();
            if password.is_empty()
                || password_confirm.is_empty()
                || name.is_empty()
                || hostname.is_empty()
            {
                show_msg(s, "Please fill in all the fields.");
                return;
            }
            if password != password_confirm {
                show_msg(s, "Password and confirm password do not match.");
                return;
            }
            let mut config = config.clone();
            config.password = Some(Arc::new(password));
            config.user = Some(Arc::new(name));
            config.hostname = Some(hostname);
            config.locale = Some(Arc::new(locale));
            show_summary(s, config);
        }),
    );
}

fn show_summary(siv: &mut Cursive, config: InstallConfig) {
    let mut path = String::new();
    let mut fs = String::new();
    let config_copy = config.clone();
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
                config.user.unwrap()
            )),
            "Confirmation",
            None,
        )
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Install", move |s| {
            s.pop_layer();
            start_install(s, config_copy.clone());
        }),
    );
}

fn start_install(siv: &mut Cursive, config: InstallConfig) {
    siv.pop_layer();
    let counter = Counter::new(0);
    let counter_clone = counter.clone();
    let mut status_message = TextView::new("");
    let status_text = Arc::new(status_message.get_shared_content());
    siv.add_layer(wrap_in_dialog(
        LinearLayout::vertical()
            .child(TextView::new(
                "Please wait while the installation is taking place.",
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
    let install_thread = thread::spawn(move || begin_install(tx, config));
    thread::spawn(move || loop {
        if let Ok(progress) = rx.recv() {
            match progress {
                super::InstallProgress::Pending(msg, pct) => {
                    counter_clone.set(pct);
                    status_text.set_content(msg);
                }
                super::InstallProgress::Finished => cb_sink.send(Box::new(show_finished)).unwrap(),
            }
        } else {
            let err = install_thread.join().unwrap().unwrap_err();
            cb_sink
                .send(Box::new(move |s| {
                    show_error(s, &err.to_string());
                }))
                .unwrap();
            return;
        }
    });
}

fn show_finished(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        wrap_in_dialog(
            TextView::new("All done!\nYou can continue playing around by pressing Quit button."),
            "All Done",
            None,
        )
        .button("Reboot", |s| {
            install::sync_and_reboot().ok();
            s.quit();
        })
        .button("Quit", |s| s.quit()),
    );
}

pub fn tui_main() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(TextView::new("Welcome to AOSC OS installer!"))
            .title("Welcome")
            .button("Start", |s| {
                let config = InstallConfig {
                    variant: None,
                    partition: None,
                    mirror: None,
                    user: None,
                    password: None,
                    hostname: None,
                    locale: None,
                };
                select_variant(s, config)
            })
            .padding_lrtb(2, 2, 1, 1),
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
