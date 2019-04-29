extern crate serde_derive;

use super::schema::users;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub id: &'a str,
  pub name: &'a str,
  pub email: &'a str,
}
