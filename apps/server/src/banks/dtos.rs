use crate::courses::CourseId;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_question", skip_on_field_errors = false))]
pub struct QuestionInput {
	#[validate(length(
		min = 1,
		max = 5000,
		message = "Prompt must be between 1 and 5000 characters"
	))]
	pub prompt: String,

	#[validate(length(min = 2, max = 5, message = "There must be between 2 and 5 options"))]
	pub options: Vec<String>,
	pub answer_index: usize,
	#[serde(default)]
	#[validate(length(max = 5000, message = "Justification must be at most 5000 characters"))]
	pub justification: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionBankDto {
	pub course_id: CourseId,
	#[validate(length(
		min = 1,
		max = 120,
		message = "Name must be between 1 and 120 characters"
	))]
	pub name: String,
	#[validate(length(
		min = 1,
		max = 500,
		message = "There must be at least 1 and at most 500 questions"
	))]
	#[validate(nested)]
	pub questions: Vec<QuestionInput>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestionInput {
	#[validate(length(
		min = 1,
		max = 5000,
		message = "Prompt must be between 1 and 5000 characters"
	))]
	pub prompt: String,

	#[validate(length(min = 2, max = 5, message = "There must be between 2 and 5 options"))]
	pub options: Vec<String>,
	/// `None` keeps the current justification, `Some(None)` clears it.
	#[serde(
		default,
		with = "serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	#[validate(length(max = 5000, message = "Justification must be at most 5000 characters"))]
	pub justification: Option<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_update_has_fields", skip_on_field_errors = false))]
pub struct UpdateQuestionBankDto {
	#[validate(length(
		min = 1,
		max = 120,
		message = "Name must be between 1 and 120 characters"
	))]
	pub name: Option<String>,
	#[validate(length(
		min = 1,
		max = 500,
		message = "There must be at least 1 and at most 500 questions"
	))]
	#[validate(nested)]
	pub questions: Option<Vec<UpdateQuestionInput>>,
}

fn validate_question(question: &QuestionInput) -> Result<(), ValidationError> {
	if question.answer_index < question.options.len() {
		return Ok(());
	}

	let mut err = ValidationError::new("invalid_answer_index");
	err.message = Some("answerIndex must be a valid option index".into());
	Err(err)
}

fn validate_update_has_fields(dto: &UpdateQuestionBankDto) -> Result<(), ValidationError> {
	if dto.name.is_some() || dto.questions.is_some() {
		return Ok(());
	}

	let mut err = ValidationError::new("empty_update");
	err.message = Some("At least one field must be provided".into());
	Err(err)
}

#[cfg(test)]
mod tests {
	use super::UpdateQuestionInput;
	use validator::Validate;

	fn parse(json: &str) -> UpdateQuestionInput {
		serde_json::from_str(json).expect("valid question payload")
	}

	#[test]
	fn justification_distinguishes_missing_null_and_value() {
		let base = r#""prompt": "P", "options": ["a", "b"]"#;

		assert_eq!(parse(&format!("{{{base}}}")).justification, None);
		assert_eq!(
			parse(&format!(r#"{{{base}, "justification": null}}"#)).justification,
			Some(None)
		);
		assert_eq!(
			parse(&format!(r#"{{{base}, "justification": "J"}}"#)).justification,
			Some(Some("J".to_owned()))
		);
	}

	#[test]
	fn justification_update_rejects_text_over_limit() {
		let input = |len: usize| UpdateQuestionInput {
			prompt: "P".to_owned(),
			options: vec!["a".to_owned(), "b".to_owned()],
			justification: Some(Some("x".repeat(len))),
		};

		assert!(input(5000).validate().is_ok());
		assert!(input(5001).validate().is_err());
	}
}
