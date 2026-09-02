//! Payment workflow domain types.
//!
//! This crate builds on exact monetary primitives from `paykit-money` and models payment
//! lifecycle rules that should not be repeated independently by API, processor, or persistence
//! layers.
//!
//! Caller-specific acceptance rules, such as payment amount limits, are exposed as policy
//! boundaries so applications can keep business configuration outside the core payment value.

mod payment;

pub use paykit_money::PaymentAmount;
pub use payment::{
    Payment, PaymentAction, PaymentActionResult, PaymentAmountPolicy, PaymentCurrencyPolicy,
    PaymentId, PaymentIdError, PaymentMethodType, PaymentStatus, PaymentTransitionError,
};
