pub mod new_session;
pub mod cloud;

pub enum Tab {
    NewSession,
    Cloud,  
}

impl PartialEq for Tab{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}