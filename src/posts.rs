use diesel;
use diesel::prelude::*;
use super::schema::posts;
use super::models::{Post, InsertablePost};

pub fn create_post(connection: &PgConnection, post: Post) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(&InsertablePost::from_post(post))
        .get_result::<Post>(connection)
}
