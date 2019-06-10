use super::schema::posts;
use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String,
    pub published: bool
}

impl InsertablePost {
    pub fn from_post(post: Post) -> InsertablePost {
        InsertablePost {
            title: post.title,
            body: post.body,
            published: post.published
        }
    }
}



