// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, PartialEq, Clone)]

pub struct TicketTitle(String);

impl TicketTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            return Err("The title cannot be empty".to_string());
        }
        if value.len() > 50 {
            return Err("The title cannot be longer than 50 bytes".to_string());
        }

        Ok(TicketTitle(value.to_string()))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        value.as_str().try_into()
    }
}
