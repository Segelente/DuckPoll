use crate::poll::{Poll, Question, Vote};

use sqlx::{Any, Pool, Postgres, Row, SqlitePool};
async fn get_questions_from_database(pool: Pool<Postgres>, poll_id: i32) -> Vec<Question> {
    let questions_raw = sqlx::query("SELECT * FROM question WHERE poll_id = $1")
        .bind(poll_id)
        .fetch_all(&pool)
        .await
        .unwrap();
    let questions: Vec<Question> = questions_raw
        .iter()
        .map(|row| {
            let text: String = row.get("text");
            let option1: String = row.get("option1");
            let option2: String = row.get("option2");
            let option3: String = row.get("option3");
            Question::new(text, vec![option1, option2, option3])
        })
        .collect();
    questions
}

pub(crate) async fn get_poll_from_database(pool: Pool<Postgres>, poll_id: i32) -> Poll {
    let poll_row_raw = sqlx::query("SELECT * FROM poll WHERE id = $1")
        .bind(&poll_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let _id: i32 = poll_row_raw.get("id");
    let title: String = poll_row_raw.get("title");

    let questions: Vec<Question> = get_questions_from_database(pool.clone(), poll_id).await;

    Poll { title, questions }
}

pub(crate) async fn insert_poll(pool: Pool<Postgres>, poll: &Poll) -> i32 {
    // Insert the poll into the `poll` table
    let poll_id: i32 = sqlx::query_scalar("INSERT INTO poll (title) VALUES ($1) RETURNING id")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await
        .unwrap();

    // Insert each question and its options into the `question_option` table
    for question in &poll.questions {
        let question_id: i32 = sqlx::query_scalar("INSERT INTO question (poll_id, text, option1, option2, option3) VALUES (?, ?, ?, ?, ?) RETURNING id")
            .bind(&poll_id)
            .bind(&question.text)
            .bind(&question.answers[0])
            .bind(&question.answers[1])
            .bind(&question.answers[2])
            .fetch_one(&pool)
            .await.unwrap();
    }
    poll_id
}

#[allow(dead_code)]
async fn create_tables(pool: &SqlitePool) -> Result<(), std::io::Error> {
    // Create the `poll` table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS poll (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .unwrap();

    // Create the `question` table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS question (
            id INTEGER PRIMARY KEY,
            poll_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY (poll_id) REFERENCES poll (id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .unwrap();
    Ok(())
}

pub(crate) async fn get_votes_from_db(pool: Pool<Postgres>, poll_id: i32) -> Vec<Vote> {
    let votes_row_raw = sqlx::query("SELECT * FROM vote WHERE poll_id = $1")
        .bind(poll_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let votes: Vec<Vote> = votes_row_raw
        .iter()
        .map(|row| {
            let question_title: String = row.get("question_title");
            let option1_votes: i32 = row.get("option1_votes");
            let option2_votes: i32 = row.get("option2_votes");
            let option3_votes: i32 = row.get("option3_votes");
            Vote::new(
                question_title,
                vec![option1_votes, option2_votes, option3_votes],
            )
        })
        .collect();

    votes
}
pub(crate) async fn save_votes_to_db(pool: Pool<Postgres>, poll_id: i32, vote: Vote) {
    // Insert the vote into the `vote` table
    let _vote_id: i32 = sqlx::query_scalar("INSERT INTO vote (poll_id, question_title, option1_votes, option2_votes, option3_votes) VALUES ($1, $2, $3, $4, $5) RETURNING id")
        .bind(&poll_id)
        .bind(&vote.question_title)
        .bind(&vote.option_votes[0])
        .bind(&vote.option_votes[1])
        .bind(&vote.option_votes[2])
        .fetch_one(&pool)
        .await.unwrap();
}
