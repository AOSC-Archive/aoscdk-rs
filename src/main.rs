mod disks;
mod network;
mod parser;

use cursive::align::HAlign;
use cursive::views::{Dialog, DummyView, LinearLayout, Panel, ScrollView, SelectView, TextView};
use cursive::Cursive;
use number_prefix::NumberPrefix;

fn show_error(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::around(TextView::new(msg))
            .title("Error")
            .button("Exit", |s| s.quit()),
    );
}

#[inline]
fn human_size(size: u64) -> String {
    let result = match NumberPrefix::binary(size as f64) {
        NumberPrefix::Standalone(bytes) => format!("{} B", bytes),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}B", n, prefix),
    };

    result
}

fn prepare(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(
            "Fetching required information...\nThis can take a while...",
        ))
        .title("Progress"),
    );
    siv.refresh();
    let recipe = network::fetch_recipe();
    if recipe.is_err() {
        show_error(siv, "Could not download recipe information");
        return;
    }
    let recipe = recipe.unwrap();
    let mirrors = network::fetch_mirrors();
    if mirrors.is_err() {
        show_error(siv, "Could not download mirrors information");
        return;
    }
    let mirrors = mirrors.unwrap();
    let partitions = disks::list_partitions();
    siv.pop_layer();
    draw_config(siv, mirrors, recipe, partitions);
}

fn draw_config(
    siv: &mut Cursive,
    mirrors: network::MirrorList,
    variants: Vec<network::VariantEntry>,
    partitions: Vec<disks::Partition>,
) {
    let mut config_view = LinearLayout::vertical();

    let mut variant_list = SelectView::new().h_align(HAlign::Left);
    for variant in variants {
        variant_list.add_item(
            format!(
                "{} (Released {}, Download size {})",
                variant.name, variant.date, variant.size
            ),
            variant,
        );
    }
    let variant_view = LinearLayout::vertical()
        .child(TextView::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        ))
        .child(DummyView {})
        .child(variant_list);
    let variant_view = Panel::new(variant_view).title("Variant");
    config_view.add_child(variant_view);
    config_view.add_child(DummyView {});

    let mut repo_list = SelectView::new().h_align(HAlign::Left);
    let mirror_list = mirrors.mirrors;
    for mirror in mirror_list {
        repo_list.add_item(format!("{} ({})", mirror.name, mirror.region), mirror)
    }
    let repo_view = LinearLayout::vertical()
        .child(TextView::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        ))
        .child(DummyView {})
        .child(repo_list);
    let repo_view = Panel::new(repo_view).title("Repositories");
    config_view.add_child(repo_view);
    config_view.add_child(DummyView {});

    let mut disk_list = SelectView::new().h_align(HAlign::Left);
    for part in partitions {
        let path_name;
        if let Some(path) = part.path {
            path_name = path.to_string_lossy().to_string();
        } else {
            path_name = "?".to_owned();
        }
        disk_list.add_item(
            format!(
                "{} ({}, {})",
                path_name,
                part.fs_type.unwrap_or("?".to_owned()),
                human_size(part.size)
            ),
            path_name,
        );
    }
    let dest_view = LinearLayout::vertical()
        .child(TextView::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit\nLine 2",
        ))
        .child(DummyView {})
        .child(disk_list);
    let dest_view = Panel::new(dest_view).title("Destination");
    config_view.add_child(dest_view);
    config_view.add_child(DummyView {});
    siv.add_layer(
        Dialog::around(ScrollView::new(config_view))
            .button("Continue", |_| {})
            .padding_lrtb(2, 2, 1, 1)
            .title("AOSC OS Installation"),
    );
}

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(TextView::new("Welcome to AOSC OS installer!"))
            .title("Welcome")
            .button("Start", |s| prepare(s)),
    );
    siv.run();
}
