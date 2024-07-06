use crate::ManifoldError;

pub struct Manifold {
    pub field1: String,
    pub field2: i32,
}

impl Manifold {
    pub fn builder() -> ManifoldBuilder {
        ManifoldBuilder::default()
    }

    pub fn apply(&self, input: String) -> String {
        format!("{}-{}", self.field1, input)
    }
}

#[derive(Default)]
pub struct ManifoldBuilder {
    field1: Option<String>,
    field2: Option<i32>,
}

impl ManifoldBuilder {
    pub fn field1(mut self, value: String) -> Self {
        self.field1 = Some(value);
        self
    }

    pub fn field2(mut self, value: i32) -> Self {
        self.field2 = Some(value);
        self
    }

    pub fn compile(self) -> Result<Manifold, ManifoldError> {
        Ok(Manifold {
            field1: self.field1.ok_or("field1 is required")?,
            field2: self.field2.ok_or("field2 is required")?,
        })
    }
}
