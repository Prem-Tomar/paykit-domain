//! Payment workflow domain types.
//!
//! This crate builds on exact monetary primitives from `paykit-money` and models payment
//! lifecycle rules that should not be repeated independently by API, processor, or persistence
//! layers.

mod payment;

pub use paykit_money::PaymentAmount;
pub use payment::{
    Payment, PaymentAction, PaymentActionResult, PaymentId, PaymentIdError, PaymentMethodType,
    PaymentStatus, PaymentTransitionError,
};
