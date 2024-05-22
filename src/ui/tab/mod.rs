pub mod new_session;
pub mod cloud;
pub mod api;

#[derive(PartialEq)]
pub enum Tab {
    API,
    NewSession,
    Cloud,
}
