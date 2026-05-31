#[derive(Debug, Clone, PartialEq)]
pub struct TagId(pub i64);

impl std::ops::Deref for TagId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TagId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
