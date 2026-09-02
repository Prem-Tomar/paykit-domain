use std::fmt;

/// A caller-provided payment identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PaymentId {
    value: String,
}

impl PaymentId {
    /// Creates a payment identifier from caller-provided text.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentIdError::Empty`] when the identifier is empty.
    /// Returns [`PaymentIdError::HasSurroundingWhitespace`] when trimming would change it.
    pub fn new(value: impl Into<String>) -> Result<Self, PaymentIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PaymentIdError::Empty);
        }

        if value.trim() != value {
            return Err(PaymentIdError::HasSurroundingWhitespace);
        }

        Ok(Self { value })
    }

    /// Returns the payment identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// An error returned when constructing a [`PaymentId`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentIdError {
    /// Payment identifiers must contain at least one character.
    Empty,
    /// Payment identifiers must not contain leading or trailing whitespace.
    HasSurroundingWhitespace,
}

impl fmt::Display for PaymentIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("payment id must not be empty"),
            Self::HasSurroundingWhitespace => {
                formatter.write_str("payment id must not contain surrounding whitespace")
            }
        }
    }
}

impl std::error::Error for PaymentIdError {}
