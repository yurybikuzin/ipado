use super::*;

// mod menu_panel;
mod user_panel;
// use user_panel::*;

// mod control_panel;
// use control_panel::*;
//
pub fn render(user: Arc<User>) -> Dom {
    html!("header",{
        .child(user_panel::render(user))
        // .child(menu_panel::render())
        // .child(control_panel())
        // .child(render::guest_content::h2())
    })
}
