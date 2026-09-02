use paykit_money::PaymentAmount;

/// Caller-owned policy for deciding whether a payment amount is allowed.
///
/// [`PaymentAmount`] already guarantees that the wrapped money value is strictly positive.
/// This trait is for workflow-specific rules such as merchant, product, rail, or regional amount
/// limits. Those rules stay outside this crate, and each caller keeps ownership of its own typed
/// rejection reason.
pub trait PaymentAmountPolicy {
    /// The caller-owned rejection type returned when the policy rejects an amount.
    type Rejection;

    /// Validates a borrowed payment amount.
    ///
    /// # Errors
    ///
    /// Returns the caller's [`Self::Rejection`] when the amount is not accepted by this policy.
    fn validate(&self, amount: &PaymentAmount) -> Result<(), Self::Rejection>;
}
