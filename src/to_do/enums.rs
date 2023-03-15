use serde::ser::{Serialize, Serializer};

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            Self::Done => "DONE".to_string(),
            Self::Pending => "PENDING".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::Done,
            "PENDING" => TaskStatus::Pending,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stringify().as_str())
    }
}
