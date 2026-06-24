pub struct CourseServices;

impl CourseServices {
    async fn validate_link(&self, _url: &str) -> Result<(), LinkError> {
        Ok(())
    }

    async fn validate_object_key(&self, _key: &str) -> Result<(), ObjectError> {
        Ok(())
    }
}

pub struct LinkError;
pub struct ObjectError;
