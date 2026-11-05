use crate::ui::components::input_board::InputBoard;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub struct AgentPage {
    input_board: Entity<InputBoard>,
}

impl Render for AgentPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input_board = self.input_board.read(cx);
        let textarea_state = input_board.textarea.read(cx);
        let text = textarea_state.value();
        let row_count = textarea_state.display_map().wrap_row_count();
        div()
            .relative()
            .flex()
            .size_full()
            .flex_col()
            .child(
                div().absolute().inset_0().child(
                    img(ImageSource::from(std::path::Path::new(
                        "/home/shen/Pictures/Paper/3.jpg",
                    )))
                    .size_full()
                    .object_fit(ObjectFit::Cover),
                ),
            )
            .child(div().size_full().absolute().inset_1().bg(Rgba {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 0.8,
            }))
            .when(row_count > 1, |div| {
                div.child(
                    gpui_kit::div()
                        .absolute()
                        .w(px(450.))
                        .h(px(600.))
                        .bottom(px(70.))
                        .right_1()
                        .bg(Rgba {
                            r: 0.2,
                            g: 0.2,
                            b: 0.2,
                            a: 0.8,
                        })
                        .rounded(px(10.))
                        .p(px(10.))
                        .border_1()
                        .border_color(Rgba {
                            r: 0.7,
                            g: 0.3,
                            b: 0.5,
                            a: 1.0,
                        })
                        .child(
                            gpui_kit::div().size_full().overflow_y_scrollbar().child(
                                gpui_kit::div()
                                    .id("content")
                                    .w_full()
                                    .whitespace_normal()
                                    .text_size(px(20.))
                                    .text_color(Rgba {
                                        r: 0.8,
                                        g: 0.2,
                                        b: 0.5,
                                        a: 1.0,
                                    })
                                    .child(text.to_string()),
                            ),
                        ),
                )
            })
            .child(div().size_full())
            .child(self.input_board.clone())
    }
}

impl AgentPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_board = cx.new(|cx| InputBoard::new(window, cx));
        AgentPage { input_board }
    }
}
