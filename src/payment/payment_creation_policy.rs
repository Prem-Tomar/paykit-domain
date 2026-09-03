use paykit_money::{Currency, PaymentAmount};

use crate::PaymentMethodType;

pub enum PaymentCreationPolicyRejection<A, C, M> {
    Amount(A),
    Currency(C),
    PaymentMethod(M),
}

#[allow(unused)]
pub struct PaymentCreationPolicySet<A, C, M> {
    amount_policy: A,
    currency_policy: C,
    payment_method_policy: M,
}

impl PaymentCreationPolicySet<PaymentAmount, Currency, PaymentMethodType> {
    pub fn new(amount: Paym, currency: Currency, method_type: PaymentMethodType) -> Self {
        Self {
            amount_policy: amount,
            currency_policy: currency,
            payment_method_policy: method_type,
        }
    }
}
