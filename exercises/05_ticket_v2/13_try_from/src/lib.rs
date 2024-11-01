// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::fmt;
//use std::path::Display;
use crate::Status::{InProgress, ToDo, Done};
use thiserror::Error;
#[derive(Error, Debug)]
#[error("{invalid_status} is not a valid status")]
struct MyError
{
    invalid_status: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}
impl TryFrom<String> for Status {
    type Error = MyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_lowercase()=="todo"
        {
            return Ok(ToDo);
        }
        else if value.to_lowercase()=="inprogress"
        {
            return Ok(InProgress);
        }
        else if value.to_lowercase()=="done"
        {
            return Ok(Done);
        }
        return Err(MyError{invalid_status:value});
    }
}

impl TryFrom<&str> for Status {
    type Error = MyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_string().to_lowercase()=="todo"
        {
            return Ok(ToDo);
        }
        else if value.to_string().to_lowercase()=="inprogress"
        {
            return Ok(InProgress);
        }
        else if value.to_string().to_lowercase()=="done"
        {
            return Ok(Done);
        }
        return Err(MyError{invalid_status:value.to_string()});
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
