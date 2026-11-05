use crate::ui::pages::agent::AgentPage;
use gpui_kit::base::{NavMotion, NavStack, NavStackState};
use gpui_kit::*;

pub struct App {
    nav_stack: Entity<NavStackState>,
    agent: Entity<AgentPage>,
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let agent_page = cx.new(|cx| AgentPage::new(window, cx));
        let nav = cx.new(|_cx| NavStackState::new());
        nav.update(cx, |stack, cx| {
            stack.push(agent_page.clone(), NavMotion::Immediate, cx);
        });
        Self {
            nav_stack: nav,
            agent: agent_page,
        }
    }
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        NavStack::new(&self.nav_stack)
            .size_full()
            .item(|page, _, _| {
                div()
                    .size_full()
                    .bg(Rgba {
                        r: 0.02,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    })
                    .child(page)
                    .into_any_element()
            })
    }
}
