use anyhow::Result;
use chrono::NaiveDateTime;

pub struct MusicSearchProgress {
    pub id: u32,
    pub music_genre_id: u32,
    pub position: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MusicSearchProgress {
    pub async fn find_by_music_genre_id(
        db_pool: &sqlx::MySqlPool,
        music_genre_id: u32,
    ) -> Result<Self> {
        let result = sqlx::query_as!(
            MusicSearchProgress,
            r#"
                SELECT
                  id, music_genre_id, position, created_at, updated_at
                FROM
                  music_search_progresses
                WHERE
                  music_genre_id = ?
            "#,
            music_genre_id
        )
        .fetch_one(db_pool)
        .await?;

        Ok(result)
    }
}
