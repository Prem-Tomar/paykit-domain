#[derive(Clone, Eq, Debug, PartialEq)]
pub enum PaymentMethodType {
    CARD,
    UPI,
    BANK_TRANSFER,
}
