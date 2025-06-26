use crate::domain::{Note, Time, Transpose, Waveform};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema, Debug, Deserialize)]
pub struct TB303Pattern {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Option<Uuid>,
    #[schema(example = "First pattern")]
    pub name: String,
    #[schema(example = "Phuture")]
    pub author: Option<String>,
    #[schema(example = "Acid trax")]
    pub title: Option<String>,
    #[schema(example = "This is a pattern")]
    pub description: Option<String>,
    #[schema(example = 120)]
    pub tempo: Option<i32>,
    #[schema(example = "sawtooth")]
    pub waveform: Option<String>,
    #[schema(example = false)]
    pub triplets: Option<bool>,
    #[schema(example = 50)]
    pub tuning: Option<i32>,
    #[schema(example = 50)]
    pub cut_off_freq: Option<i32>,
    #[schema(example = 50)]
    pub resonance: Option<i32>,
    #[schema(example = 50)]
    pub env_mod: Option<i32>,
    #[schema(example = 50)]
    pub decay: Option<i32>,
    #[schema(example = 50)]
    pub accent: Option<i32>,
    #[schema(example = true)]
    pub is_public: Option<bool>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,
    pub steps: Vec<TB303Step>,
}

#[derive(Serialize, ToSchema, Debug, Deserialize)]
pub struct CreateTB303Pattern {
    #[schema(example = "First pattern")]
    pub name: String,
    #[schema(example = "Phuture")]
    pub author: Option<String>,
    #[schema(example = "Acid trax")]
    pub title: Option<String>,
    #[schema(example = "This is a pattern")]
    pub description: Option<String>,
    #[schema(example = 120)]
    pub tempo: Option<i32>,
    #[schema(example = "sawtooth")]
    pub waveform: Option<Waveform>,
    #[schema(example = false)]
    pub triplets: Option<bool>,
    #[schema(example = 50)]
    pub tuning: Option<i32>,
    #[schema(example = 50)]
    pub cut_off_freq: Option<i32>,
    #[schema(example = 50)]
    pub resonance: Option<i32>,
    #[schema(example = 50)]
    pub env_mod: Option<i32>,
    #[schema(example = 50)]
    pub decay: Option<i32>,
    #[schema(example = 50)]
    pub accent: Option<i32>,
    #[schema(example = true)]
    pub is_public: Option<bool>,
    pub steps: Vec<CreateTB303Step>,
}

#[derive(Serialize, ToSchema, Debug, Deserialize)]
pub struct TB303Step {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = 1)]
    pub number: i32,
    #[schema(example = "C")]
    pub note: Option<String>,
    #[schema(example = "up")]
    pub transpose: Option<String>,
    #[schema(example = "note")]
    pub time: Option<String>,
    #[schema(example = true)]
    pub accent: Option<bool>,
    #[schema(example = false)]
    pub slide: Option<bool>,
}

#[derive(Serialize, ToSchema, Debug, Deserialize)]
pub struct CreateTB303Step {
    #[schema(example = 1)]
    pub number: i32,
    #[schema(example = "C")]
    pub note: Option<Note>,
    #[schema(example = "up")]
    pub transpose: Option<Transpose>,
    #[schema(example = "note")]
    pub time: Time,
    #[schema(example = true)]
    pub accent: Option<bool>,
    #[schema(example = false)]
    pub slide: Option<bool>,
}

#[derive(Serialize, sqlx::FromRow, Default, ToSchema)]
pub struct TB303PatternSummary {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub pattern_id: Uuid,
    #[schema(example = "Phuture")]
    pub author: Option<String>,
    #[schema(example = "Acid Trax")]
    pub title: Option<String>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub updated_at: DateTime<Utc>,
}
