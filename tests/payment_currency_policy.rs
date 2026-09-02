use paykit_domain::PaymentCurrencyPolicy;
use paykit_money::Currency;

#[derive(Debug, Eq, PartialEq)]
enum CurrencyPolicyRejection {
    UnsupportedCurrency { code: String, minor_units: u8 },
}

struct ExactCurrencyPolicy {
    code: &'static str,
    minor_units: u8,
}

impl PaymentCurrencyPolicy for ExactCurrencyPolicy {
    type Rejection = CurrencyPolicyRejection;

    fn validate(&self, currency: &Currency) -> Result<(), Self::Rejection> {
        if currency.code() == self.code && currency.minor_units() == self.minor_units {
            return Ok(());
        }

        Err(CurrencyPolicyRejection::UnsupportedCurrency {
            code: currency.code().to_owned(),
            minor_units: currency.minor_units(),
        })
    }
}

fn currency(code: &str, minor_units: u8) -> Currency {
    Currency::new(code, minor_units).expect("test currency should be valid")
}

#[test]
fn external_currency_policy_can_accept_a_currency_definition() {
    let policy = ExactCurrencyPolicy {
        code: "USD",
        minor_units: 2,
    };
    let usd = currency("USD", 2);

    assert_eq!(policy.validate(&usd), Ok(()));
    assert_eq!(usd.code(), "USD");
    assert_eq!(usd.minor_units(), 2);
}

#[test]
fn external_currency_policy_preserves_typed_rejection_for_custom_currency() {
    let policy = ExactCurrencyPolicy {
        code: "USD",
        minor_units: 2,
    };
    let custom = currency("XTS", 4);

    assert_eq!(
        policy.validate(&custom),
        Err(CurrencyPolicyRejection::UnsupportedCurrency {
            code: "XTS".to_owned(),
            minor_units: 4,
        })
    );
    assert_eq!(custom.code(), "XTS");
    assert_eq!(custom.minor_units(), 4);
}

#[test]
fn currency_policy_can_reject_same_code_with_different_minor_unit_scale() {
    let policy = ExactCurrencyPolicy {
        code: "USD",
        minor_units: 2,
    };
    let same_code_different_scale = currency("USD", 3);

    assert_eq!(
        policy.validate(&same_code_different_scale),
        Err(CurrencyPolicyRejection::UnsupportedCurrency {
            code: "USD".to_owned(),
            minor_units: 3,
        })
    );
}
