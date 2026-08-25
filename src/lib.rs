//! Payment workflow domain types.
//!
//! This crate builds on exact monetary primitives from `paykit-money` and models payment
//! lifecycle rules that should not be repeated independently by API, processor, or persistence
//! layers.

use std::fmt;

pub use paykit_money::PaymentAmount;

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

/// The controlled lifecycle status of a payment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PaymentStatus {
    /// The payment exists but has not been authorized.
    Created,
    /// Funds have been authorized and can be captured or voided.
    Authorized,
    /// The authorized payment has been captured.
    Captured,
    /// The created payment was cancelled before authorization.
    Cancelled,
    /// The authorization was voided before capture.
    Voided,
}

/// A lifecycle operation attempted against a payment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PaymentOperation {
    /// Move a created payment to authorized.
    Authorize,
    /// Move an authorized payment to captured.
    Capture,
    /// Move a created payment to cancelled.
    Cancel,
    /// Move an authorized payment to voided.
    Void,
}

/// A payment with a checked lifecycle status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    id: PaymentId,
    amount: PaymentAmount,
    status: PaymentStatus,
}

impl Payment {
    /// Creates a payment in [`PaymentStatus::Created`].
    #[must_use]
    pub const fn new(id: PaymentId, amount: PaymentAmount) -> Self {
        Self {
            id,
            amount,
            status: PaymentStatus::Created,
        }
    }

    /// Returns the payment identifier.
    #[must_use]
    pub const fn id(&self) -> &PaymentId {
        &self.id
    }

    /// Returns the validated payment amount.
    #[must_use]
    pub const fn amount(&self) -> &PaymentAmount {
        &self.amount
    }

    /// Returns the current lifecycle status.
    #[must_use]
    pub const fn status(&self) -> PaymentStatus {
        self.status
    }

    /// Records that the payment has been authorized.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// created.
    pub fn authorize(&mut self) -> Result<(), PaymentTransitionError> {
        self.transition(PaymentOperation::Authorize, PaymentStatus::Authorized)
    }

    /// Records that the authorized payment has been captured.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// authorized.
    pub fn capture(&mut self) -> Result<(), PaymentTransitionError> {
        self.transition(PaymentOperation::Capture, PaymentStatus::Captured)
    }

    /// Records that the created payment has been cancelled.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// created.
    pub fn cancel(&mut self) -> Result<(), PaymentTransitionError> {
        self.transition(PaymentOperation::Cancel, PaymentStatus::Cancelled)
    }

    /// Records that the authorized payment has been voided.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// authorized.
    pub fn void(&mut self) -> Result<(), PaymentTransitionError> {
        self.transition(PaymentOperation::Void, PaymentStatus::Voided)
    }

    fn transition(
        &mut self,
        operation: PaymentOperation,
        next_status: PaymentStatus,
    ) -> Result<(), PaymentTransitionError> {
        if is_allowed_transition(operation, self.status) {
            self.status = next_status;
            return Ok(());
        }

        Err(PaymentTransitionError::InvalidTransition {
            operation,
            current_status: self.status,
        })
    }
}

fn is_allowed_transition(operation: PaymentOperation, current_status: PaymentStatus) -> bool {
    matches!(
        (operation, current_status),
        (PaymentOperation::Authorize, PaymentStatus::Created)
            | (PaymentOperation::Cancel, PaymentStatus::Created)
            | (PaymentOperation::Capture, PaymentStatus::Authorized)
            | (PaymentOperation::Void, PaymentStatus::Authorized)
    )
}

/// An error returned when a payment lifecycle transition is rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentTransitionError {
    /// The attempted operation is not valid for the payment's current status.
    InvalidTransition {
        /// The operation the caller attempted to record.
        operation: PaymentOperation,
        /// The status the payment had when the operation was attempted.
        current_status: PaymentStatus,
    },
}

impl fmt::Display for PaymentTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition {
                operation,
                current_status,
            } => write!(
                formatter,
                "invalid payment transition: {operation:?} cannot be applied when status is {current_status:?}"
            ),
        }
    }
}

impl std::error::Error for PaymentTransitionError {}
