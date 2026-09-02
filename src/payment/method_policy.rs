use super::method::PaymentMethodType;

/// Caller-owned policy for deciding whether a payment method is accepted by a payment workflow.
///
/// [`PaymentMethodType`] defines the payment-method categories understood by this crate. This
/// trait lets applications choose which of those categories they accept while retaining ownership
/// of their typed rejection reason.
pub trait PaymentMethodPolicy {
    /// The caller-owned rejection type returned when the policy rejects a payment method.
    type Rejection;

    /// Validates a payment method category.
    ///
    /// # Errors
    ///
    /// Returns the caller's [`Self::Rejection`] when the method is not accepted by this policy.
    fn validate(&self, payment_method: PaymentMethodType) -> Result<(), Self::Rejection>;
}
