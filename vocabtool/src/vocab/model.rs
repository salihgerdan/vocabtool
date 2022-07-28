use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::{FromRow, Row};
use crate::vocab::tokenize;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct TextCreateRequest {
    pub text: String,
    pub lang: String,
    pub user: i32,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Text {
    pub id: i32,
    pub text: String,
    pub tokenized: String,
    pub lang: String,
    pub user: i32,
}

// implementation of Actix Responder for Text struct so we can return Text from action handler
impl Responder for Text {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementation for Text struct, functions to read/write/update
// and delete texts from database
impl Text {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<Text>> {
        let mut texts = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, text, tokenized, lang, user
                    FROM texts
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            texts.push(Text {
                id: rec.id,
                text: rec.text,
                tokenized: rec.tokenized,
                lang: rec.lang,
                user: rec.user,
            });
        }

        Ok(texts)
    }

    pub async fn find_by_id(id: i32, pool: &MySqlPool) -> Result<Text> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM texts WHERE id = ?
                "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Text {
                id: rec.id,
                text: rec.text,
                tokenized: rec.tokenized,
                lang: rec.lang,
                user: rec.user,
        })
    }

    pub async fn create(textreq: TextCreateRequest, pool: &MySqlPool) -> Result<Text> {
        let tokenized = tokenize::process(&textreq.text, &textreq.lang).await.expect("Failure in tokenization");
        let mut tx = pool.begin().await?;
        let text = sqlx::query("INSERT INTO texts (text, tokenized, lang, user) VALUES (?, ?, ?, ?) \
                               RETURNING id, text, tokenized, lang, user")
            .bind(&textreq.text)
            .bind(&tokenized)
            .bind(&textreq.lang)
            .bind(textreq.user)
            .map(|row: MySqlRow| { Text {
                    id: row.get(0),
                    text: row.get(1),
                    tokenized: row.get(2),
                    lang: row.get(3),
                    user: row.get(4)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(text)
    }

    pub async fn update(id: i32, textreq: TextCreateRequest, pool: &MySqlPool) -> Result<Text> {
        let tokenized = tokenize::process(&textreq.text, &textreq.lang).await.expect("Failure in tokenization");
        let mut tx = pool.begin().await.unwrap();
        let text = sqlx::query("UPDATE texts SET text = ?, tokenized = ?, lang = ? WHERE id = ? \
                               RETURNING id, text, tokenized, lang, user")
            .bind(&textreq.text)
            .bind(&tokenized)
            .bind(&textreq.lang)
            .bind(id)
            .map(|row: MySqlRow| {
                Text {
                    id: row.get(0),
                    text: row.get(1),
                    tokenized: row.get(2),
                    lang: row.get(3),
                    user: row.get(4)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(text)
    }

    pub async fn delete(id: i32, pool: &MySqlPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM texts WHERE id = ?")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}

#[derive(Serialize, FromRow)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub state: i32,
    pub notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct WordRequest {
    pub word: String,
    pub state: i32,
    pub notes: String,
    pub lang: String,
    pub user: i32,
}

// implementation of Actix Responder for Word struct so we can return Word from action handler
impl Responder for Word {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Word {
    pub async fn wordsiknow(user:i32, lang: &str, pool: &MySqlPool) -> Result<Vec<Word>> {
        let mut words = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, word, state, notes
                    FROM words
                WHERE user = ? AND lang = ?
            "#, user, lang
        )
        .fetch_all(pool)
        .await?;
    
        for rec in recs {
            words.push(Word {
                id: rec.id,
                word: rec.word,
                state: rec.state,
                notes: rec.notes,
            });
        }
    
        Ok(words)
    }

    pub async fn create(wordreq: WordRequest, pool: &MySqlPool) -> Result<Word> {
        let mut tx = pool.begin().await?;
        let word = sqlx::query("INSERT INTO words (word, state, notes, lang, user) VALUES (?, ?, ?, ?, ?) \
                               RETURNING id, word, state, notes")
            .bind(&wordreq.word)
            .bind(wordreq.state)
            .bind(&wordreq.notes)
            .bind(&wordreq.lang)
            .bind(wordreq.user)
            .map(|row: MySqlRow| { Word {
                    id: row.get(0),
                    word: row.get(1),
                    state: row.get(2),
                    notes: row.get(3),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(word)
    }

    pub async fn update(id: i32, wordreq: WordRequest, pool: &MySqlPool) -> Result<Word> {
        let mut tx = pool.begin().await?;
        let word = sqlx::query("UPDATE words SET word = ?, state = ?, notes = ?  WHERE id = ? \
                               RETURNING id, word, state, notes")
            .bind(&wordreq.word)
            .bind(wordreq.state)
            .bind(&wordreq.notes)
            .bind(id)
            .map(|row: MySqlRow| { Word {
                    id: row.get(0),
                    word: row.get(1),
                    state: row.get(2),
                    notes: row.get(3),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(word)
    }
}
