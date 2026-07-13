use crate::aggregates::course::service::CourseServices;

/// External dependencies `Faq::handle` needs beyond its own state — currently
/// just a way to check that a `course_id` refers to a real course before a
/// FAQ can be created against it, mirroring `LinkAggregateServices`.
pub struct FaqAggregateServices {
    pub course: CourseServices,
}
