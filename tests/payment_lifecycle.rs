use paykit_domain::{
    Payment, PaymentId, PaymentIdError, PaymentOperation, PaymentStatus, PaymentTransitionError,
};
use paykit_money::{Currency, Money, PaymentAmount};

fn currency(code: &str, minor_units: u8) -> Currency {
    Currency::new(code, minor_units).expect("test currency should be valid")
}

fn payment_amount(minor_units: i128) -> PaymentAmount {
    PaymentAmount::new(Money::from_minor_units(minor_units, currency("USD", 2)))
        .expect("test amount should be positive")
}

fn payment() -> Payment {
    Payment::new(
        PaymentId::new("pay_001").expect("test id should be valid"),
        payment_amount(1_000),
    )
}

#[test]
fn payment_id_accepts_caller_provided_identifier() {
    let id = PaymentId::new("pay_001").expect("id should be valid");

    assert_eq!(id.as_str(), "pay_001");
}

#[test]
fn payment_id_rejects_empty_identifier() {
    assert_eq!(PaymentId::new(""), Err(PaymentIdError::Empty));
}

#[test]
fn payment_id_rejects_leading_or_trailing_whitespace() {
    assert_eq!(
        PaymentId::new(" pay_001"),
        Err(PaymentIdError::HasSurroundingWhitespace)
    );
    assert_eq!(
        PaymentId::new("pay_001 "),
        Err(PaymentIdError::HasSurroundingWhitespace)
    );
}

#[test]
fn new_payment_starts_created_and_preserves_inputs() {
    let id = PaymentId::new("pay_001").expect("id should be valid");
    let amount = payment_amount(1_000);

    let payment = Payment::new(id.clone(), amount.clone());

    assert_eq!(payment.id(), &id);
    assert_eq!(payment.amount(), &amount);
    assert_eq!(payment.status(), PaymentStatus::Created);
}

#[test]
fn created_payment_can_be_authorized() {
    let mut payment = payment();

    payment.authorize().expect("created payment can authorize");

    assert_eq!(payment.status(), PaymentStatus::Authorized);
}

#[test]
fn created_payment_can_be_cancelled() {
    let mut payment = payment();

    payment.cancel().expect("created payment can cancel");

    assert_eq!(payment.status(), PaymentStatus::Cancelled);
}

#[test]
fn authorized_payment_can_be_captured() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    payment.capture().expect("authorized payment can capture");

    assert_eq!(payment.status(), PaymentStatus::Captured);
}

#[test]
fn authorized_payment_can_be_voided() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");

    payment.void().expect("authorized payment can void");

    assert_eq!(payment.status(), PaymentStatus::Voided);
}

#[test]
fn capture_before_authorization_is_rejected_and_state_is_unchanged() {
    let mut payment = payment();
    let before = payment.clone();

    let error = payment
        .capture()
        .expect_err("created payment cannot be captured");

    assert_eq!(
        error,
        PaymentTransitionError::InvalidTransition {
            operation: PaymentOperation::Capture,
            current_status: PaymentStatus::Created,
        }
    );
    assert_eq!(payment, before);
}

#[test]
fn void_before_authorization_is_rejected_and_state_is_unchanged() {
    let mut payment = payment();
    let before = payment.clone();

    let error = payment
        .void()
        .expect_err("created payment cannot be voided");

    assert_eq!(
        error,
        PaymentTransitionError::InvalidTransition {
            operation: PaymentOperation::Void,
            current_status: PaymentStatus::Created,
        }
    );
    assert_eq!(payment, before);
}

#[test]
fn authorize_after_authorization_is_rejected_and_state_is_unchanged() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");
    let before = payment.clone();

    let error = payment
        .authorize()
        .expect_err("authorized payment cannot be authorized again");

    assert_eq!(
        error,
        PaymentTransitionError::InvalidTransition {
            operation: PaymentOperation::Authorize,
            current_status: PaymentStatus::Authorized,
        }
    );
    assert_eq!(payment, before);
}

#[test]
fn cancel_after_authorization_is_rejected_and_state_is_unchanged() {
    let mut payment = payment();
    payment.authorize().expect("created payment can authorize");
    let before = payment.clone();

    let error = payment
        .cancel()
        .expect_err("authorized payment cannot be cancelled");

    assert_eq!(
        error,
        PaymentTransitionError::InvalidTransition {
            operation: PaymentOperation::Cancel,
            current_status: PaymentStatus::Authorized,
        }
    );
    assert_eq!(payment, before);
}

#[test]
fn authorize_after_cancellation_is_rejected_and_state_is_unchanged() {
    let mut payment = payment();
    payment.cancel().expect("created payment can cancel");
    let before = payment.clone();

    let error = payment
        .authorize()
        .expect_err("cancelled payment cannot be authorized");

    assert_eq!(
        error,
        PaymentTransitionError::InvalidTransition {
            operation: PaymentOperation::Authorize,
            current_status: PaymentStatus::Cancelled,
        }
    );
    assert_eq!(payment, before);
}

#[test]
fn terminal_statuses_reject_every_operation_and_keep_state_unchanged() {
    let terminal_builders: [fn() -> Payment; 3] = [
        || {
            let mut payment = payment();
            payment.authorize().expect("created payment can authorize");
            payment.capture().expect("authorized payment can capture");
            payment
        },
        || {
            let mut payment = payment();
            payment.cancel().expect("created payment can cancel");
            payment
        },
        || {
            let mut payment = payment();
            payment.authorize().expect("created payment can authorize");
            payment.void().expect("authorized payment can void");
            payment
        },
    ];

    for build_terminal_payment in terminal_builders {
        for operation in [
            PaymentOperation::Authorize,
            PaymentOperation::Capture,
            PaymentOperation::Cancel,
            PaymentOperation::Void,
        ] {
            let mut payment = build_terminal_payment();
            let before = payment.clone();
            let current_status = payment.status();

            let error = match operation {
                PaymentOperation::Authorize => payment.authorize(),
                PaymentOperation::Capture => payment.capture(),
                PaymentOperation::Cancel => payment.cancel(),
                PaymentOperation::Void => payment.void(),
            }
            .expect_err("terminal payment should reject every operation");

            assert_eq!(
                error,
                PaymentTransitionError::InvalidTransition {
                    operation,
                    current_status,
                }
            );
            assert_eq!(payment, before);
        }
    }
}

#[test]
fn custom_currency_amount_is_preserved_through_successful_transitions() {
    let currency = currency("XTS", 4);
    let amount = PaymentAmount::new(Money::from_minor_units(12_345, currency.clone()))
        .expect("amount should be positive");
    let mut payment = Payment::new(
        PaymentId::new("pay_custom").expect("id should be valid"),
        amount,
    );

    payment.authorize().expect("created payment can authorize");
    payment.void().expect("authorized payment can void");

    assert_eq!(payment.amount().money().minor_units(), 12_345);
    assert_eq!(payment.amount().money().currency(), &currency);
    assert_eq!(payment.status(), PaymentStatus::Voided);
}

#[test]
fn displays_payment_id_errors_stably() {
    assert_eq!(
        PaymentIdError::Empty.to_string(),
        "payment id must not be empty"
    );
    assert_eq!(
        PaymentIdError::HasSurroundingWhitespace.to_string(),
        "payment id must not contain surrounding whitespace"
    );
}

#[test]
fn displays_transition_errors_stably() {
    let error = PaymentTransitionError::InvalidTransition {
        operation: PaymentOperation::Capture,
        current_status: PaymentStatus::Created,
    };

    assert_eq!(
        error.to_string(),
        "invalid payment transition: Capture cannot be applied when status is Created"
    );
}
