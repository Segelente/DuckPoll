use crate::poll::{Poll, Question, Vote};

use sqlx::{Row, SqlitePool};

pub(crate) async fn get_id_from_database(pool: SqlitePool, poll: &Poll) -> i32 {
    let poll_id: i32 = sqlx::query_scalar("SELECT id FROM poll WHERE title = ?")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await
        .unwrap();
    poll_id
}
async fn get_questions_from_database(pool: SqlitePool, poll_id: i32) -> Vec<Question> {
    let questions_raw = sqlx::query("SELECT * FROM question WHERE poll_id = ?")
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

pub(crate) async fn get_poll_from_database(pool: SqlitePool, poll_id: i32) -> Poll {
    let poll_row_raw = sqlx::query("SELECT * FROM poll WHERE id = ?")
        .bind(&poll_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let _id: i32 = poll_row_raw.get("id");
    let title: String = poll_row_raw.get("title");

    let questions: Vec<Question> = get_questions_from_database(pool.clone(), poll_id).await;

    Poll { title, questions }
}

pub(crate) async fn insert_poll(pool: SqlitePool, poll: &Poll) -> Result<(), std::io::Error> {
    // Insert the poll into the `poll` table
    let poll_id: i32 = sqlx::query_scalar("INSERT INTO poll (title) VALUES (?) RETURNING id")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await
        .unwrap();

    // Insert each question and its options into the `question_option` table
    for question in &poll.questions {
        let question_id: i32 = sqlx::query_scalar("INSERT INTO question (poll_id, text, option1, option2, option3) VALUES (?, ?, ?, ?, ?) RETURNING id")
            .bind(&poll_id)
            .bind(&question.text)
            .bind(&question.options[0])
            .bind(&question.options[1])
            .bind(&question.options[2])
            .fetch_one(&pool)
            .await.unwrap();
    }

    Ok(())
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

pub(crate) async fn get_votes_from_db(pool: SqlitePool, poll_id: i32) -> Vec<Vote> {
    let votes_row_raw = sqlx::query("SELECT * FROM vote WHERE poll_id = ?")
        .bind(poll_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let votes: Vec<Vote> = votes_row_raw
        .iter()
        .map(|row| {
            let question_title: String = row.get("text");
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
pub(crate) async fn save_votes_to_db(pool: SqlitePool, poll_id: i32, vote: Vote) {
    // Insert the vote into the `vote` table
    let _vote_id: i32 = sqlx::query_scalar("INSERT INTO vote (poll_id, question_title, option1_votes, option2_votes, option3_votes) VALUES (?, ?, ?, ?, ?) RETURNING id")
        .bind(&poll_id)
        .bind(&vote.question_title)
        .bind(&vote.option_votes[0])
        .bind(&vote.option_votes[1])
        .bind(&vote.option_votes[2])
        .fetch_one(&pool)
        .await.unwrap();
}
