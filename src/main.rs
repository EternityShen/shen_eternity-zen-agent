use gpui_kit::*;
use shen_eternity_zen_agent::ui::app;

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| app::App::new(window, cx))
        })
        .expect("窗口打开失败");
    });
}
