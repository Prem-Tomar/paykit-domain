use paykit_domain::PaymentAmountPolicy;
use paykit_money::{Currency, Money, PaymentAmount};

#[derive(Debug, Eq, PartialEq)]
enum AmountPolicyRejection {
    BelowMinimum {
        minimum_minor_units: i128,
        actual_minor_units: i128,
    },
    AboveMaximum {
        maximum_minor_units: i128,
        actual_minor_units: i128,
    },
}

struct MinorUnitRangePolicy {
    minimum_minor_units: i128,
    maximum_minor_units: i128,
}

impl PaymentAmountPolicy for MinorUnitRangePolicy {
    type Rejection = AmountPolicyRejection;

    fn validate(&self, amount: &PaymentAmount) -> Result<(), Self::Rejection> {
        let actual_minor_units = amount.money().minor_units();

        if actual_minor_units < self.minimum_minor_units {
            return Err(AmountPolicyRejection::BelowMinimum {
                minimum_minor_units: self.minimum_minor_units,
                actual_minor_units,
            });
        }

        if actual_minor_units > self.maximum_minor_units {
            return Err(AmountPolicyRejection::AboveMaximum {
                maximum_minor_units: self.maximum_minor_units,
                actual_minor_units,
            });
        }

        Ok(())
    }
}

fn payment_amount(minor_units: i128, code: &str, scale: u8) -> PaymentAmount {
    let currency = Currency::new(code, scale).expect("test currency should be valid");
    let money = Money::from_minor_units(minor_units, currency);

    PaymentAmount::new(money).expect("test payment amount should be positive")
}

#[test]
fn external_amount_policy_can_accept_a_payment_amount() {
    let policy = MinorUnitRangePolicy {
        minimum_minor_units: 500,
        maximum_minor_units: 10_000,
    };
    let amount = payment_amount(1_500, "USD", 2);

    assert_eq!(policy.validate(&amount), Ok(()));
    assert_eq!(amount.money().minor_units(), 1_500);
}

#[test]
fn external_amount_policy_preserves_typed_rejection() {
    let policy = MinorUnitRangePolicy {
        minimum_minor_units: 500,
        maximum_minor_units: 10_000,
    };
    let amount = payment_amount(100, "USD", 2);

    assert_eq!(
        policy.validate(&amount),
        Err(AmountPolicyRejection::BelowMinimum {
            minimum_minor_units: 500,
            actual_minor_units: 100,
        })
    );
    assert_eq!(amount.money().minor_units(), 100);
}

#[test]
fn amount_policy_can_reject_without_consuming_or_changing_currency_definition() {
    let policy = MinorUnitRangePolicy {
        minimum_minor_units: 500,
        maximum_minor_units: 10_000,
    };
    let amount = payment_amount(25_000, "XTS", 4);

    assert_eq!(
        policy.validate(&amount),
        Err(AmountPolicyRejection::AboveMaximum {
            maximum_minor_units: 10_000,
            actual_minor_units: 25_000,
        })
    );
    assert_eq!(amount.money().minor_units(), 25_000);
    assert_eq!(amount.money().currency().code(), "XTS");
    assert_eq!(amount.money().currency().minor_units(), 4);
}
