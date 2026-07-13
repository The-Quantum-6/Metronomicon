use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
	contribution::{command::Contribution, aggregate::ContributionStatus},
};

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ContributionDetailView {
	pub status: ContributionStatus,
	pub contribution_id: Uuid,
	pub course_id: Uuid,
	pub contribution: Option<Contribution>,
}
