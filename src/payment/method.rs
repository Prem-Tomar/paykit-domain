/// The payment method category attached to a payment workflow.
#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub enum PaymentMethodType {
    /// A payment funded by a card.
    Card,
    /// A payment funded through India's Unified Payments Interface.
    Upi,
    /// A payment funded by a bank transfer.
    BankTransfer,
}
