use crate::prelude::*;

#[derive(IntoElement)]
pub struct ListSeparator;

impl RenderOnce for ListSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .h(px(1.5))
            .w_full()
            .my(DynamicSpacing::Base06.rems(cx))
            .bg(cx.theme().colors().border_variant)
    }
}
