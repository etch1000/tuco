// Represents a course dictated by a tutor.
#[derive(Serialize, Debug, Clone)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub name: String,
    pub posted_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// Represents a course for the CREATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// Represents a course for the UPDATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {}
