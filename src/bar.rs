use penrose::{x::XConn, Color};

pub const FONT: &str = "Iosevka";
pub const POINT_SIZE: u8 = 12;
pub const HEIGHT_PX: u32 = 24;

pub const DEEP_OCEAN: u32 = 0x030F1CFF;
pub const WHITE: u32 = 0xFFFFFFFF;
pub const AQUA: u32 = 0x86A1B2FF;
pub const SEA: u32 = 0x4DA2FFFF;
const MAX_ACTIVE_WINDOW_CHARS: usize = 250;

use penrose_ui::{
    bar::{
        widgets::{
            sys::interval::{amixer_volume, current_date_and_time, wifi_network},
            ActiveWindowName, CurrentLayout, Workspaces,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
use std::time::Duration;

pub fn render<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = AQUA.into();
    let empty_ws: Color = SEA.into();

    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(DEEP_OCEAN.into()),
        padding: (2, 2),
    };

    let padded_style = TextStyle {
        padding: (4, 2),
        ..style
    };

    StatusBar::try_new(
        Position::Top,
        HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        FONT,
        POINT_SIZE,
        vec![
            Box::new(Workspaces::new(style, SEA, empty_ws)),
            Box::new(CurrentLayout::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                TextStyle {
                    bg: Some(highlight),
                    padding: (6, 4),
                    ..style
                },
                true,
                false,
            )),
            Box::new(battery_summary(padded_style, Duration::new(5, 0))),
            Box::new(current_date_and_time(padded_style, Duration::new(5, 0))),
        ],
    )
}

pub fn battery_summary(
    style: TextStyle,
    interval: Duration,
) -> penrose_ui::bar::widgets::IntervalText {
    penrose_ui::bar::widgets::IntervalText::new(
        style,
        move || {
            penrose_ui::bar::widgets::sys::helpers::read_sys_file("BAT0", "capacity")
                .map(|bat| format!("{bat}%"))
        },
        interval,
        false,
        true,
    )
}
