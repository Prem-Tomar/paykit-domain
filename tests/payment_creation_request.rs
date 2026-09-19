use paykit_domain::{PaymentCreationRequest, PaymentId, PaymentMethodType};
use paykit_money::{Currency, Money, PaymentAmount};

fn payment_amount(minor_units: i128, code: &str, scale: u8) -> PaymentAmount {
    let currency = Currency::new(code, scale).expect("test currency should be valid");
    let money = Money::from_minor_units(minor_units, currency);

    PaymentAmount::new(money).expect("test payment amount should be positive")
}

#[test]
fn creation_request_preserves_every_supplied_value() {
    let request = PaymentCreationRequest::new(
        PaymentId::new("pay_creation_request").expect("test id should be valid"),
        payment_amount(25_000, "XTS", 4),
        PaymentMethodType::Upi,
    );

    assert_eq!(request.id().as_str(), "pay_creation_request");
    assert_eq!(request.amount().money().minor_units(), 25_000);
    assert_eq!(request.amount().money().currency().code(), "XTS");
    assert_eq!(request.amount().money().currency().minor_units(), 4);
    assert_eq!(request.payment_method(), PaymentMethodType::Upi);
}

#[test]
fn copied_payment_method_does_not_consume_the_request() {
    let request = PaymentCreationRequest::new(
        PaymentId::new("pay_reusable_request").expect("test id should be valid"),
        payment_amount(1_500, "USD", 2),
        PaymentMethodType::BankTransfer,
    );

    let payment_method = request.payment_method();

    assert_eq!(payment_method, PaymentMethodType::BankTransfer);
    assert_eq!(request.payment_method(), PaymentMethodType::BankTransfer);
    assert_eq!(request.id().as_str(), "pay_reusable_request");
    assert_eq!(request.amount().money().minor_units(), 1_500);
}
