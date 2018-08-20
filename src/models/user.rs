use std::fmt;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}
