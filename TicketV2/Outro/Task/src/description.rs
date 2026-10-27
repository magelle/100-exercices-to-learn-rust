// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("The description cannot be empty".to_string());
        }
        if value.len() > 500 {
            return Err("The description cannot be longer than 500 bytes".to_string());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}
