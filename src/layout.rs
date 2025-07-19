use penrose::builtin::layout::{CenteredMain, MainAndStack, Monocle};
use penrose::core::layout::{Layout, LayoutStack};
use penrose::extensions::layout::{Conditional, Tatami};
use penrose::stack;

pub fn render() -> LayoutStack {
    stack!(flex_main(), Monocle::boxed(), Tatami::boxed(0.6, 0.1))
}

fn flex_main() -> Box<dyn Layout> {
    Conditional::boxed(
        "FlexMain",
        MainAndStack::default(),
        CenteredMain::default(),
        |_, r| r.w <= 1400,
    )
}
