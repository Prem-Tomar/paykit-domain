use super::{PaymentId, PaymentMethodType};
use paykit_money::PaymentAmount;

#[derive(Debug)]
pub struct PaymentCreationRequest {
    id: PaymentId,
    amount: PaymentAmount,
    payment_method: PaymentMethodType,
}

impl PaymentCreationRequest {
    pub fn new(id: PaymentId, amount: PaymentAmount, payment_method: PaymentMethodType) -> Self {
        Self {
            id,
            amount,
            payment_method,
        }
    }

    pub fn id(&self) -> &PaymentId {
        &self.id
    }

    pub fn amount(&self) -> &PaymentAmount {
        &self.amount
    }

    pub fn payment_method(&self) -> PaymentMethodType {
        self.payment_method
    }
}
