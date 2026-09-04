use paykit_money::PaymentAmount;

use crate::{PaymentAmountPolicy, PaymentCurrencyPolicy, PaymentMethodPolicy, PaymentMethodType};

pub enum PaymentCreationPolicyRejection<A, C, M> {
    Amount(A),
    Currency(C),
    PaymentMethod(M),
}

pub type PaymentCreationPolicyValidationResult<A, C, M> =
    Result<(), PaymentCreationPolicyRejection<A, C, M>>;

pub struct PaymentCreationPolicySet<A, C, M> {
    amount_policy: A,
    currency_policy: C,
    payment_method_policy: M,
}

impl<A, C, M> PaymentCreationPolicySet<A, C, M> {
    pub fn new(amount_policy: A, currency_policy: C, payment_method_policy: M) -> Self {
        Self {
            amount_policy,
            currency_policy,
            payment_method_policy,
        }
    }
}

impl<A, C, M> PaymentCreationPolicySet<A, C, M>
where
    A: PaymentAmountPolicy,
    C: PaymentCurrencyPolicy,
    M: PaymentMethodPolicy,
{
    pub fn validate(
        &self,
        amount: &PaymentAmount,
        payment_method: PaymentMethodType,
    ) -> PaymentCreationPolicyValidationResult<A::Rejection, C::Rejection, M::Rejection> {
        self.amount_policy
            .validate(amount)
            .map_err(PaymentCreationPolicyRejection::Amount)?;

        self.currency_policy
            .validate(amount.money().currency())
            .map_err(PaymentCreationPolicyRejection::Currency)?;

        self.payment_method_policy
            .validate(payment_method)
            .map_err(PaymentCreationPolicyRejection::PaymentMethod)?;

        Ok(())
    }
}
