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
    /// On success, returns evidence describing the transition from [`PaymentStatus::Created`]
    /// to [`PaymentStatus::Authorized`].
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// created.
    pub fn authorize(&mut self) -> Result<PaymentTransition, PaymentTransitionError> {
        self.transition(PaymentOperation::Authorize)
    }

    /// Records that the authorized payment has been captured.
    ///
    /// On success, returns evidence describing the transition from [`PaymentStatus::Authorized`]
    /// to [`PaymentStatus::Captured`].
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// authorized.
    pub fn capture(&mut self) -> Result<PaymentTransition, PaymentTransitionError> {
        self.transition(PaymentOperation::Capture)
    }

    /// Records that the created payment has been cancelled.
    ///
    /// On success, returns evidence describing the transition from [`PaymentStatus::Created`]
    /// to [`PaymentStatus::Cancelled`].
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// created.
    pub fn cancel(&mut self) -> Result<PaymentTransition, PaymentTransitionError> {
        self.transition(PaymentOperation::Cancel)
    }

    /// Records that the authorized payment has been voided.
    ///
    /// On success, returns evidence describing the transition from [`PaymentStatus::Authorized`]
    /// to [`PaymentStatus::Voided`].
    ///
    /// # Errors
    ///
    /// Returns [`PaymentTransitionError::InvalidTransition`] when the payment is not currently
    /// authorized.
    pub fn void(&mut self) -> Result<PaymentTransition, PaymentTransitionError> {
        self.transition(PaymentOperation::Void)
    }

    fn transition(
        &mut self,
        operation: PaymentOperation,
    ) -> Result<PaymentTransition, PaymentTransitionError> {
        let previous_status = self.status;
        let Some(new_status) = next_status(operation, previous_status) else {
            return Err(PaymentTransitionError::InvalidTransition {
                operation,
                current_status: previous_status,
            });
        };

        self.status = new_status;
        Ok(PaymentTransition {
            previous_status,
            new_status,
            operation,
        })
    }
}

fn next_status(
    operation: PaymentOperation,
    current_status: PaymentStatus,
) -> Option<PaymentStatus> {
    match (operation, current_status) {
        (PaymentOperation::Authorize, PaymentStatus::Created) => Some(PaymentStatus::Authorized),
        (PaymentOperation::Cancel, PaymentStatus::Created) => Some(PaymentStatus::Cancelled),
        (PaymentOperation::Capture, PaymentStatus::Authorized) => Some(PaymentStatus::Captured),
        (PaymentOperation::Void, PaymentStatus::Authorized) => Some(PaymentStatus::Voided),
        _ => None,
    }
}

/// Evidence that a payment lifecycle transition completed successfully.
///
/// This value records the applied operation and the payment statuses immediately before and after
/// the in-memory state change. It does not represent processor confirmation, persistence, or a
/// durable domain event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentTransition {
    previous_status: PaymentStatus,
    new_status: PaymentStatus,
    operation: PaymentOperation,
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

#[cfg(test)]
mod tests {
    use paykit_money::{Currency, Money};

    use super::*;

    fn payment() -> Payment {
        let currency = Currency::new("USD", 2).expect("test currency should be valid");
        let amount = PaymentAmount::new(Money::from_minor_units(1_000, currency))
            .expect("test amount should be positive");

        Payment::new(
            PaymentId::new("pay_transition").expect("test id should be valid"),
            amount,
        )
    }

    #[test]
    fn authorization_returns_exact_transition_evidence() {
        let mut payment = payment();

        let transition = payment.authorize().expect("created payment can authorize");

        assert_eq!(
            transition,
            PaymentTransition {
                previous_status: PaymentStatus::Created,
                new_status: PaymentStatus::Authorized,
                operation: PaymentOperation::Authorize,
            }
        );
        assert_eq!(payment.status(), PaymentStatus::Authorized);
    }

    #[test]
    fn cancellation_returns_exact_transition_evidence() {
        let mut payment = payment();

        let transition = payment.cancel().expect("created payment can cancel");

        assert_eq!(
            transition,
            PaymentTransition {
                previous_status: PaymentStatus::Created,
                new_status: PaymentStatus::Cancelled,
                operation: PaymentOperation::Cancel,
            }
        );
        assert_eq!(payment.status(), PaymentStatus::Cancelled);
    }

    #[test]
    fn capture_returns_exact_transition_evidence() {
        let mut payment = payment();
        payment.authorize().expect("created payment can authorize");

        let transition = payment.capture().expect("authorized payment can capture");

        assert_eq!(
            transition,
            PaymentTransition {
                previous_status: PaymentStatus::Authorized,
                new_status: PaymentStatus::Captured,
                operation: PaymentOperation::Capture,
            }
        );
        assert_eq!(payment.status(), PaymentStatus::Captured);
    }

    #[test]
    fn void_returns_exact_transition_evidence() {
        let mut payment = payment();
        payment.authorize().expect("created payment can authorize");

        let transition = payment.void().expect("authorized payment can void");

        assert_eq!(
            transition,
            PaymentTransition {
                previous_status: PaymentStatus::Authorized,
                new_status: PaymentStatus::Voided,
                operation: PaymentOperation::Void,
            }
        );
        assert_eq!(payment.status(), PaymentStatus::Voided);
    }
}
