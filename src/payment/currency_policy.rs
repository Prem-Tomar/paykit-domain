use paykit_money::Currency;

/// Caller-owned policy for deciding whether a currency is accepted by a payment workflow.
///
/// `paykit-money` keeps [`Currency`] open and data-driven, so this trait is where applications
/// can apply their own accepted-currency rules without turning currency definitions into a
/// built-in enum or static catalog.
pub trait PaymentCurrencyPolicy {
    /// The caller-owned rejection type returned when the policy rejects a currency.
    type Rejection;

    /// Validates a borrowed currency definition.
    ///
    /// # Errors
    ///
    /// Returns the caller's [`Self::Rejection`] when the currency is not accepted by this policy.
    fn validate(&self, currency: &Currency) -> Result<(), Self::Rejection>;
}
