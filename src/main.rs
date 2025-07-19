//! My Windows Manager with Penrose
use penrose::core::WindowManager;
use penrose::extensions::hooks::add_ewmh_hooks;
use penrose::x11rb::RustConn;

use mwm::bar;
use mwm::keybinds;

use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let wm = WindowManager::new(
        add_ewmh_hooks(mwm::config()),
        keybinds::settings(),
        HashMap::new(),
        RustConn::new()?,
    )?;

    let wm_with_bar = bar::render()?.add_to(wm);
    wm_with_bar.run()?;

    Ok(())
}
