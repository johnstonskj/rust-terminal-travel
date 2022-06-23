/*!
One-line description.

More detailed description, with

# Example

*/

use serde::Deserialize;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueSource {
    parameter: String,
    example: String,
    pointer: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    status: u32,
    code: u64,
    title: String,
    detail: Option<String>,
    source: Option<IssueSource>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    errors: Vec<Issue>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn errors(&self) -> impl Iterator<Item = &Issue> {
        self.errors.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl Issue {
    pub fn status(&self) -> u32 {
        self.status
    }

    pub fn code(&self) -> u64 {
        self.code
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn detail(&self) -> Option<&String> {
        self.detail.as_ref()
    }

    pub fn source(&self) -> Option<&IssueSource> {
        self.source.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl IssueSource {
    pub fn parameter(&self) -> &String {
        &self.parameter
    }

    pub fn example(&self) -> &String {
        &self.example
    }

    pub fn pointer(&self) -> Option<&String> {
        self.pointer.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
