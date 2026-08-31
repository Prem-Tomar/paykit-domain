use paykit_domain::{Payment, PaymentId, PaymentOperation, PaymentStatus};
use paykit_money::{Currency, Money, PaymentAmount};

fn payment() -> Payment {
    let currency = Currency::new("USD", 2).expect("test currency should be valid");
    let amount = PaymentAmount::new(Money::from_minor_units(1_000, currency))
        .expect("test amount should be positive");

    Payment::new(
        PaymentId::new("pay_transition").expect("test id should be valid"),
        amount,
    )
}

#[test]
fn authorization_returns_exact_transition_evidence() {
    let mut payment = payment();

    let transition = payment.authorize().expect("created payment can authorize");

    assert_eq!(transition.operation(), PaymentOperation::Authorize);
    assert_eq!(transition.previous_status(), PaymentStatus::Created);
    assert_eq!(transition.resulting_status(), PaymentStatus::Authorized);
    assert_eq!(payment.status(), PaymentStatus::Authorized);
}

#[test]
fn cancellation_returns_exact_transition_evidence() {
    let mut payment = payment();

    let transition = payment.cancel().expect("created payment can cancel");

    assert_eq!(transition.operation(), PaymentOperation::Cancel);
    assert_eq!(transition.previous_status(), PaymentStatus::Created);
    assert_eq!(transition.resulting_status(), PaymentStatus::Cancelled);
    assert_eq!(payment.status(), PaymentStatus::Cancelled);
}

#[test]
fn capture_returns_exact_transition_evidence() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    let transition = payment.capture().expect("authorized payment can capture");

    assert_eq!(transition.operation(), PaymentOperation::Capture);
    assert_eq!(transition.previous_status(), PaymentStatus::Authorized);
    assert_eq!(transition.resulting_status(), PaymentStatus::Captured);
    assert_eq!(payment.status(), PaymentStatus::Captured);
}

#[test]
fn void_returns_exact_transition_evidence() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    let transition = payment.void().expect("authorized payment can void");

    assert_eq!(transition.operation(), PaymentOperation::Void);
    assert_eq!(transition.previous_status(), PaymentStatus::Authorized);
    assert_eq!(transition.resulting_status(), PaymentStatus::Voided);
    assert_eq!(payment.status(), PaymentStatus::Voided);
}
