#![allow(unused_imports)]
pub mod upsert_survey;

use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::{
    api::common::CommonQueryResponse,
    models::{
        question::{Question, QuestionAnswer, QuestionType},
        survey::{Age, Gender, ProofId, QuestionSequence, Quota, SurveyStatus, SurveySummary},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished")]
    Finished,
}

#[server(endpoint = "/v1/surveys/lists", input = GetUrl, output = Json)]
pub async fn list_surveys(
    email: String,
    _size: Option<i32>,
    _bookmark: Option<String>,
    _status: Option<Status>,
) -> Result<CommonQueryResponse<SurveySummary>, ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys/lists",);

    use crate::api::aws::ses::AuthKeyModel;
    use easy_dynamodb::error::DynamoException;

    let log = crate::utils::logger::new_api("GET", &format!("/v1/surveys/lists"));

    CommonQueryResponse::query(&log, "gsi1-index", None, Some(100), vec![("gsi1", email)]).await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSurveyResponse {
    pub survey: SurveySummary,
    pub responders: Vec<ProofId>,
    pub questions: Vec<Question>,
}

#[server(endpoint = "/v1/email/:email/surveys/:survey-id", input = GetUrl, output = Json)]
pub async fn get_survey() -> Result<GetSurveyResponse, ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys/:survey-id:");
    use axum::extract::Path;

    let Path((email, survey_id)): Path<(String, String)> = extract().await?;
    let log =
        crate::utils::logger::new_api("GET", &format!("/v1/email/{}/surveys/{}", email, survey_id));
    let cli = crate::utils::db::get(&log);

    let res_summary: Result<Option<SurveySummary>, easy_dynamodb::error::DynamoException> = cli
        .get(format!("{}#survey#{}", email, survey_id).as_str())
        .await;

    let res_question: Result<
        (Option<Vec<Question>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = cli
        .find(
            "gsi1-index",
            None,
            Some(100),
            vec![("gsi1", format!("{}#survey-question#{}", email, survey_id))],
        )
        .await;

    let survey = match res_summary {
        Ok(v) => match v {
            Some(summary) => summary,
            None => return Err(ServerFnError::ServerError(format!("not exists survey"))),
        },
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "DB summary query failed {e}"
            )))
        }
    };

    let questions: Vec<Question> = match res_question {
        Ok(v) => {
            let questions_vec = &v.0.unwrap();
            questions_vec.clone()
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "DB questions query failed {e}"
            )))
        }
    };

    Ok(GetSurveyResponse {
        survey,
        responders: vec![],
        questions,
    })
}

#[server(endpoint = "/v1/surveys/answer", input = Json, output = Json)]
pub async fn answer_survey(
    survey_id: String,
    answers: Vec<QuestionAnswer>,
) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys/:survey-id: {} {:?}", survey_id, answers);

    Ok(())
}
