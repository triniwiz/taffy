use taffy::prelude::*;

// Three `width: 33.3333%` items share one line at every container width, as in a browser:
// 1/3 rounds up in f32, so their summed widths can land just past the container's.
#[test]
fn thirds_share_one_line() {
    for tenth in 3000..14000 {
        let width = tenth as f32 / 10.0;
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let item = Style { flex_shrink: 0.0, size: Size { width: percent(0.333333333333), height: length(10.0) }, ..Default::default() };
        let items: Vec<_> = (0..3).map(|_| taffy.new_leaf(item.clone()).unwrap()).collect();
        let row = taffy
            .new_with_children(
                Style { display: Display::Flex, flex_wrap: FlexWrap::Wrap, size: Size { width: length(width), height: auto() }, ..Default::default() },
                &items,
            )
            .unwrap();
        taffy.compute_layout(row, Size::MAX_CONTENT).unwrap();
        assert_eq!(taffy.layout(items[2]).unwrap().location.y, 0.0, "third item wrapped at width {width}");
    }
}

// Overflow beyond rounding still wraps.
#[test]
fn real_overflow_still_wraps() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let item = Style { flex_shrink: 0.0, size: Size { width: length(100.05), height: length(10.0) }, ..Default::default() };
    let items: Vec<_> = (0..3).map(|_| taffy.new_leaf(item.clone()).unwrap()).collect();
    let row = taffy
        .new_with_children(
            Style { display: Display::Flex, flex_wrap: FlexWrap::Wrap, size: Size { width: length(300.0), height: auto() }, ..Default::default() },
            &items,
        )
        .unwrap();
    taffy.compute_layout(row, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(items[2]).unwrap().location.y, 10.0);
}
