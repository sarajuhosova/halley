use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Id {
    pub name: String,
    pub uuid: Uuid,
}

impl Id {
    pub fn new(name: String) -> Id {
        Id { name, uuid: Uuid::new_v4() }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
