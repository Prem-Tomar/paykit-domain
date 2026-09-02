use std::fmt;

use paykit_money::PaymentAmount;

use super::id::PaymentId;

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

/// A lifecycle action attempted against a payment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PaymentAction {
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
    pub fn authorize(&mut self) -> Result<PaymentActionResult, PaymentTransitionError> {
        self.transition(PaymentAction::Authorize)
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
    pub fn capture(&mut self) -> Result<PaymentActionResult, PaymentTransitionError> {
        self.transition(PaymentAction::Capture)
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
    pub fn cancel(&mut self) -> Result<PaymentActionResult, PaymentTransitionError> {
        self.transition(PaymentAction::Cancel)
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
    pub fn void(&mut self) -> Result<PaymentActionResult, PaymentTransitionError> {
        self.transition(PaymentAction::Void)
    }

    fn transition(
        &mut self,
        operation: PaymentAction,
    ) -> Result<PaymentActionResult, PaymentTransitionError> {
        let previous_status = self.status;
        let Some(new_status) = next_status(operation, previous_status) else {
            return Err(PaymentTransitionError::InvalidTransition {
                operation,
                current_status: previous_status,
            });
        };

        self.status = new_status;
        Ok(PaymentActionResult {
            previous_status,
            new_status,
            operation,
        })
    }
}

fn next_status(operation: PaymentAction, current_status: PaymentStatus) -> Option<PaymentStatus> {
    match (operation, current_status) {
        (PaymentAction::Authorize, PaymentStatus::Created) => Some(PaymentStatus::Authorized),
        (PaymentAction::Cancel, PaymentStatus::Created) => Some(PaymentStatus::Cancelled),
        (PaymentAction::Capture, PaymentStatus::Authorized) => Some(PaymentStatus::Captured),
        (PaymentAction::Void, PaymentStatus::Authorized) => Some(PaymentStatus::Voided),
        _ => None,
    }
}

/// Evidence that a payment lifecycle transition completed successfully.
///
/// This value records the applied action and the payment statuses immediately before and after
/// the in-memory state change. It does not represent processor confirmation, persistence, or a
/// durable domain event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentActionResult {
    previous_status: PaymentStatus,
    new_status: PaymentStatus,
    operation: PaymentAction,
}

impl PaymentActionResult {
    /// Returns the lifecycle action that produced this result.
    #[must_use]
    pub const fn action(&self) -> PaymentAction {
        self.operation
    }

    /// Returns the payment status immediately before the action succeeded.
    #[must_use]
    pub const fn previous_status(&self) -> PaymentStatus {
        self.previous_status
    }

    /// Returns the payment status produced by the successful action.
    #[must_use]
    pub const fn resulting_status(&self) -> PaymentStatus {
        self.new_status
    }
}

/// An error returned when a payment lifecycle transition is rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentTransitionError {
    /// The attempted action is not valid for the payment's current status.
    InvalidTransition {
        /// The action the caller attempted to record.
        operation: PaymentAction,
        /// The status the payment had when the action was attempted.
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
