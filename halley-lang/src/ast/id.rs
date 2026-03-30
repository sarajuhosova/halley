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
