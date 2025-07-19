use penrose::builtin::hooks::SpacingHook;
use penrose::core::Config;
use penrose::extensions::hooks::manage::SetWorkspace;
use penrose::x::query::ClassName;
use penrose::x11rb::RustConn;

pub mod bar;
pub mod keybinds;
pub mod layout;

pub const GAP_PX: u32 = 0;

pub fn config() -> Config<RustConn> {
    let manage_hook = Box::new((ClassName("obs"), SetWorkspace("1")));
    let layout_hook = Box::new(SpacingHook {
        inner_px: GAP_PX,
        outer_px: GAP_PX,
        top_px: bar::HEIGHT_PX,
        bottom_px: 0,
    });

    Config {
        default_layouts: layout::render(),
        startup_hook: None,
        layout_hook: Some(layout_hook),
        manage_hook: Some(manage_hook),
        tags: vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
        ],
        focused_border: 0x86A1B2FF.into(),
        ..Config::default()
    }
}
