use std::{cell::RefCell, rc::Rc};

use paykit_domain::{
    PaymentAmountPolicy, PaymentCreationPolicyRejection, PaymentCreationPolicySet,
    PaymentCurrencyPolicy, PaymentMethodPolicy, PaymentMethodType,
};
use paykit_money::{Currency, Money, PaymentAmount};

type Calls = Rc<RefCell<Vec<&'static str>>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    Amount,
    Currency,
    PaymentMethod,
}

#[derive(Debug, Eq, PartialEq)]
struct AmountRejection(i128);

#[derive(Debug, Eq, PartialEq)]
struct CurrencyRejection {
    code: String,
    minor_units: u8,
}

#[derive(Debug, Eq, PartialEq)]
struct MethodRejection(PaymentMethodType);

struct RecordingAmountPolicy {
    calls: Calls,
    rejected_stage: Option<Stage>,
}

impl PaymentAmountPolicy for RecordingAmountPolicy {
    type Rejection = AmountRejection;

    fn validate(&self, amount: &PaymentAmount) -> Result<(), Self::Rejection> {
        self.calls.borrow_mut().push("amount");

        if self.rejected_stage == Some(Stage::Amount) {
            return Err(AmountRejection(amount.money().minor_units()));
        }

        Ok(())
    }
}

struct RecordingCurrencyPolicy {
    calls: Calls,
    rejected_stage: Option<Stage>,
}

impl PaymentCurrencyPolicy for RecordingCurrencyPolicy {
    type Rejection = CurrencyRejection;

    fn validate(&self, currency: &Currency) -> Result<(), Self::Rejection> {
        self.calls.borrow_mut().push("currency");

        if self.rejected_stage == Some(Stage::Currency) {
            return Err(CurrencyRejection {
                code: currency.code().to_owned(),
                minor_units: currency.minor_units(),
            });
        }

        Ok(())
    }
}

struct RecordingMethodPolicy {
    calls: Calls,
    rejected_stage: Option<Stage>,
}

impl PaymentMethodPolicy for RecordingMethodPolicy {
    type Rejection = MethodRejection;

    fn validate(&self, payment_method: PaymentMethodType) -> Result<(), Self::Rejection> {
        self.calls.borrow_mut().push("payment_method");

        if self.rejected_stage == Some(Stage::PaymentMethod) {
            return Err(MethodRejection(payment_method));
        }

        Ok(())
    }
}

fn policy_set(
    calls: &Calls,
    rejected_stage: Option<Stage>,
) -> PaymentCreationPolicySet<RecordingAmountPolicy, RecordingCurrencyPolicy, RecordingMethodPolicy>
{
    PaymentCreationPolicySet::new(
        RecordingAmountPolicy {
            calls: Rc::clone(calls),
            rejected_stage,
        },
        RecordingCurrencyPolicy {
            calls: Rc::clone(calls),
            rejected_stage,
        },
        RecordingMethodPolicy {
            calls: Rc::clone(calls),
            rejected_stage,
        },
    )
}

fn payment_amount(minor_units: i128, code: &str, scale: u8) -> PaymentAmount {
    let currency = Currency::new(code, scale).expect("test currency should be valid");
    let money = Money::from_minor_units(minor_units, currency);

    PaymentAmount::new(money).expect("test payment amount should be positive")
}

#[test]
fn composed_policies_accept_in_deterministic_order() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let policies = policy_set(&calls, None);
    let amount = payment_amount(1_500, "USD", 2);

    assert!(policies.validate(&amount, PaymentMethodType::Card).is_ok());
    assert_eq!(*calls.borrow(), ["amount", "currency", "payment_method"]);
    assert_eq!(amount.money().minor_units(), 1_500);
}

#[test]
fn amount_rejection_short_circuits_and_preserves_its_type() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let policies = policy_set(&calls, Some(Stage::Amount));
    let amount = payment_amount(100, "USD", 2);

    match policies.validate(&amount, PaymentMethodType::Card) {
        Err(PaymentCreationPolicyRejection::Amount(rejection)) => {
            assert_eq!(rejection, AmountRejection(100));
        }
        _ => panic!("expected an amount-policy rejection"),
    }

    assert_eq!(*calls.borrow(), ["amount"]);
}

#[test]
fn currency_rejection_uses_the_amount_currency_and_short_circuits() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let policies = policy_set(&calls, Some(Stage::Currency));
    let amount = payment_amount(2_500, "XTS", 4);

    match policies.validate(&amount, PaymentMethodType::Upi) {
        Err(PaymentCreationPolicyRejection::Currency(rejection)) => {
            assert_eq!(
                rejection,
                CurrencyRejection {
                    code: "XTS".to_owned(),
                    minor_units: 4,
                }
            );
        }
        _ => panic!("expected a currency-policy rejection"),
    }

    assert_eq!(*calls.borrow(), ["amount", "currency"]);
}

#[test]
fn payment_method_rejection_preserves_its_type_after_prior_checks() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let policies = policy_set(&calls, Some(Stage::PaymentMethod));
    let amount = payment_amount(5_000, "USD", 2);

    match policies.validate(&amount, PaymentMethodType::BankTransfer) {
        Err(PaymentCreationPolicyRejection::PaymentMethod(rejection)) => {
            assert_eq!(rejection, MethodRejection(PaymentMethodType::BankTransfer));
        }
        _ => panic!("expected a payment-method-policy rejection"),
    }

    assert_eq!(*calls.borrow(), ["amount", "currency", "payment_method"]);
}
