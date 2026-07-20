use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ReportCommand {
    /// report an issue
    ///
    /// Requires `read` (any logged-in user)
    Create {
        issue_id: Uuid,
        course_id: Option<Uuid>,
        title: String,
        description: String,
        contact_email: Option<String>,
    },

    /// Requires `page_admin`
    ResolveReport { issue_id: Uuid },

    /// Requires `page_admin`
    ReopenReport { issue_id: Uuid },
    // Delete report
    //
    // Requires `page_admin`
    // Delete { issue_id: Uuid },
}

impl ReportCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            ReportCommand::Create { issue_id, .. } => issue_id,
            ReportCommand::ResolveReport { issue_id } => issue_id,
            ReportCommand::ReopenReport { issue_id } => issue_id,
            //ReportCommand::Delete { issue_id } => issue_id,
        }
    }
}
