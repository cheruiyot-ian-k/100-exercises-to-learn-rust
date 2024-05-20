struct Ticket {
    title: String,
    description: String,
    status: String,
}
pub fn panic_if_empty(str: &str, key: &str) {
    if str.is_empty() {
        panic!("{} cannot be empty", key);
    }
}
pub fn panic_if_chacharacter_count_exceeds(str: &str, count: usize, key: &str) {
    if str.len() > count {
        panic!("{} cannot be longer than {} characters", key, count);
    }
}
pub fn panic_if_status_does_not_mattch(str: &str) {
    match str {
        "To-Do" | "In Progress" | "Done" => println!("accepted status"),
        _ => panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed"),
    }
}
impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        panic_if_chacharacter_count_exceeds(&title, 50, "Title");
        panic_if_chacharacter_count_exceeds(&description, 500, "Description");
        panic_if_empty(&title, "Title");
        panic_if_empty(&description, "Description");
        if (description.len() > 500) {
            panic!("Description cannot be longer than 500 characters");
        }
        panic_if_status_does_not_mattch(&status);
        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 characters")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
