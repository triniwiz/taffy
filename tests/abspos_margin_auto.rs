//! Regression test: absolutely-positioned element with `inset:0` on all sides
//! plus `margin:auto` and a definite size must center on BOTH axes, even when
//! the box is larger than half the containing block on an axis.
use taffy::prelude::*;

#[test]
fn abspos_margin_auto_centers_even_when_box_larger_than_half() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let child = tree
        .new_leaf(Style {
            position: Position::Absolute,
            inset: Rect { left: length(0.0), right: length(0.0), top: length(0.0), bottom: length(0.0) },
            margin: Rect { left: auto(), right: auto(), top: auto(), bottom: auto() },
            size: Size { width: length(90.0), height: length(90.0) },
            ..Default::default()
        })
        .unwrap();

    let root = tree
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(290.0), height: length(150.0) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let l = tree.layout(child).unwrap();
    // (290-90)/2 = 100 ; (150-90)/2 = 30 — the 90px box is > half of 150.
    assert!((l.location.x - 100.0).abs() < 0.5, "x not centered: {}", l.location.x);
    assert!((l.location.y - 30.0).abs() < 0.5, "y not centered: {}", l.location.y);
}
