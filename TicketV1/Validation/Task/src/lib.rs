use std::ops::Not;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //  Panic messages: "Only `To-Do`, `In Progress`, and `Done` statuses are allowed",
    //  "Title cannot be empty", "Description cannot be empty", "Title cannot be longer than 50 bytes",
    //  "Description cannot be longer than 500 bytes"
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    pub fn new(title: String, description: String, status: String) -> Self {
        Self::assert_valid_status(&status);

        Self::assert_valid_title(&title);
        Self::assert_valid_description(&description);

        Self {
            title,
            description,
            status,
        }
    }

    fn assert_valid_status(status: &String) {
        if ["To-Do".into(), "In Progress".into(), "Done".into()]
            .contains(status)
            .not()
        {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }

    fn assert_valid_title(title: &String) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
    }

    fn assert_valid_description(description: &String) {
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
    }
}
