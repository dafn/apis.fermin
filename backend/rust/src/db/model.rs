use chrono::NaiveDateTime;
use super::schema::notes;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub content: String,
}


#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
  pub content: &'a str
}
