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
