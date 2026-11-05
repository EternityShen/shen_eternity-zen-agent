use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::*;

pub struct InputBoard {
    pub textarea: Entity<TextareaState>,
}

impl InputBoard {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let textarea = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder("输入.....")
                .auto_grow(1, 1)
        });

        Self { textarea }
    }
}

impl Render for InputBoard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(70.))
            .flex_row()
            .items_center()
            .justify_start()
            .p(px(5.))
            .bg(Rgba {
                r: 0.2,
                g: 0.2,
                b: 0.4,
                a: 0.5,
            })
            .child(
                Textarea::new(&self.textarea)
                    .w_full()
                    .h(px(35.))
                    .bg(Rgba {
                        r: 0.2,
                        g: 0.2,
                        b: 0.3,
                        a: 0.6,
                    })
                    .text_color(Rgba {
                        r: 0.8,
                        g: 0.5,
                        b: 0.6,
                        a: 1.0,
                    })
                    .border(px(1.))
                    .rounded(px(10.)),
            )
            .child(div().w(px(50.)))
            .child(
                Button::new(1)
                    .size(px(40.))
                    .bg(Rgba {
                        r: 0.2,
                        g: 0.3,
                        b: 0.4,
                        a: 1.0,
                    })
                    .custom(
                        ButtonCustomVariant::new(cx).hover(
                            Rgba {
                                r: 0.2,
                                g: 0.3,
                                b: 0.5,
                                a: 1.0,
                            }
                            .into(),
                        ),
                    )
                    .label(">")
                    .rounded(px(10.)),
            )
    }
}
