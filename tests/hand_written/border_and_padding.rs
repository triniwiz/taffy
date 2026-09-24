use taffy::prelude::*;
use taffy::style_helpers::TaffyZero;
use taffy_test_helpers::new_test_tree;

fn arr_to_rect<T: Copy>(items: [T; 4]) -> Rect<T> {
    Rect { left: items[0], right: items[1], top: items[2], bottom: items[3] }
}

#[test]
#[ignore]
fn border_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = new_test_tree();
        let node = taffy
            .new_leaf(Style {
                border: {
                    let mut lengths = [LengthPercentage::ZERO; 4];
                    lengths[i] = LengthPercentage::from_length(10.);
                    arr_to_rect(lengths)
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn padding_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = new_test_tree();
        let node = taffy
            .new_leaf(Style {
                padding: {
                    let mut lengths = [LengthPercentage::ZERO; 4];
                    lengths[i] = LengthPercentage::from_length(10.);
                    arr_to_rect(lengths)
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn border_and_padding_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = new_test_tree();
        let rect = {
            let mut lengths = [LengthPercentage::ZERO; 4];
            lengths[i] = LengthPercentage::from_length(10.);
            arr_to_rect(lengths)
        };
        let node = taffy.new_leaf(Style { border: rect, padding: rect, ..Default::default() }).unwrap();

        taffy
            .compute_layout(
                node,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn vertical_border_and_padding_percentage_values_use_available_space_correctly() {
    let mut taffy = new_test_tree();

    let node = taffy
        .new_leaf(Style {
            padding: Rect {
                left: LengthPercentage::from_percent(1.0),
                top: LengthPercentage::from_percent(1.0),
                ..Rect::zero()
            },
            ..Default::default()
        })
        .unwrap();

    taffy
        .compute_layout(node, Size { width: AvailableSpace::Definite(200.0), height: AvailableSpace::Definite(100.0) })
        .unwrap();

    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.size.width, 200.0);
    assert_eq!(layout.size.height, 200.0);
}

// An auto-width padded flex row offered 400px shrink-to-fits to 400px outer,
// not 400px minus its padding.
#[test]
fn padded_row_shrink_to_fit_clamps_against_outer_available_space() {
    let mut taffy = new_test_tree();
    let item = taffy
        .new_leaf(Style {
            size: Size { width: length(500.0), height: length(10.0) },
            min_size: Size { width: length(0.0), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let row = taffy
        .new_with_children(
            Style {
                padding: Rect { left: length(10.0), right: length(10.0), top: zero(), bottom: zero() },
                ..Default::default()
            },
            &[item],
        )
        .unwrap();
    let wrapper = taffy
        .new_with_children(
            Style { flex_direction: FlexDirection::Column, align_items: Some(AlignItems::START), ..Default::default() },
            &[row],
        )
        .unwrap();
    taffy
        .compute_layout(wrapper, Size { width: AvailableSpace::Definite(400.0), height: AvailableSpace::MaxContent })
        .unwrap();
    assert_eq!(taffy.layout(row).unwrap().size.width, 400.0);
    assert_eq!(taffy.layout(item).unwrap().size.width, 380.0);
}
