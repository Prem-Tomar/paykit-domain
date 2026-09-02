use paykit_domain::{Payment, PaymentId, PaymentMethodType};
use paykit_money::{Currency, Money, PaymentAmount};

fn payment(payment_method: PaymentMethodType) -> Payment {
    let currency = Currency::new("USD", 2).expect("test currency should be valid");
    let amount = PaymentAmount::new(Money::from_minor_units(1_000, currency))
        .expect("test amount should be positive");

    Payment::new(
        PaymentId::new("pay_method").expect("test id should be valid"),
        amount,
        payment_method,
    )
}

#[test]
fn every_supported_payment_method_can_be_attached_to_a_payment() {
    for payment_method in [
        PaymentMethodType::Card,
        PaymentMethodType::Upi,
        PaymentMethodType::BankTransfer,
    ] {
        let payment = payment(payment_method);

        assert_eq!(payment.payment_method(), payment_method);
    }
}

#[test]
fn successful_lifecycle_actions_preserve_the_payment_method() {
    let mut payment = payment(PaymentMethodType::Upi);

    payment.authorize().expect("created payment can authorize");
    payment.capture().expect("authorized payment can capture");

    assert_eq!(payment.payment_method(), PaymentMethodType::Upi);
}

#[test]
fn rejected_lifecycle_actions_preserve_the_payment_method() {
    let mut payment = payment(PaymentMethodType::BankTransfer);

    payment
        .capture()
        .expect_err("created payment cannot be captured");

    assert_eq!(payment.payment_method(), PaymentMethodType::BankTransfer);
}
