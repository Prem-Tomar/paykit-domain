use paykit_domain::{Payment, PaymentAction, PaymentId, PaymentMethodType, PaymentStatus};
use paykit_money::{Currency, Money, PaymentAmount};

fn payment() -> Payment {
    let currency = Currency::new("USD", 2).expect("test currency should be valid");
    let amount = PaymentAmount::new(Money::from_minor_units(1_000, currency))
        .expect("test amount should be positive");

    Payment::new(
        PaymentId::new("pay_action_result").expect("test id should be valid"),
        amount,
        PaymentMethodType::Card,
    )
}

#[test]
fn authorization_returns_exact_action_result() {
    let mut payment = payment();

    let result = payment.authorize().expect("created payment can authorize");

    assert_eq!(result.action(), PaymentAction::Authorize);
    assert_eq!(result.previous_status(), PaymentStatus::Created);
    assert_eq!(result.resulting_status(), PaymentStatus::Authorized);
    assert_eq!(payment.status(), PaymentStatus::Authorized);
}

#[test]
fn cancellation_returns_exact_action_result() {
    let mut payment = payment();

    let result = payment.cancel().expect("created payment can cancel");

    assert_eq!(result.action(), PaymentAction::Cancel);
    assert_eq!(result.previous_status(), PaymentStatus::Created);
    assert_eq!(result.resulting_status(), PaymentStatus::Cancelled);
    assert_eq!(payment.status(), PaymentStatus::Cancelled);
}

#[test]
fn capture_returns_exact_action_result() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    let result = payment.capture().expect("authorized payment can capture");

    assert_eq!(result.action(), PaymentAction::Capture);
    assert_eq!(result.previous_status(), PaymentStatus::Authorized);
    assert_eq!(result.resulting_status(), PaymentStatus::Captured);
    assert_eq!(payment.status(), PaymentStatus::Captured);
}

#[test]
fn void_returns_exact_action_result() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    let result = payment.void().expect("authorized payment can void");

    assert_eq!(result.action(), PaymentAction::Void);
    assert_eq!(result.previous_status(), PaymentStatus::Authorized);
    assert_eq!(result.resulting_status(), PaymentStatus::Voided);
    assert_eq!(payment.status(), PaymentStatus::Voided);
}
