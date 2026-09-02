use paykit_domain::{PaymentMethodPolicy, PaymentMethodType};

#[derive(Debug, Eq, PartialEq)]
enum MethodPolicyRejection {
    UnsupportedMethod(PaymentMethodType),
}

struct CardOnlyPolicy;

impl PaymentMethodPolicy for CardOnlyPolicy {
    type Rejection = MethodPolicyRejection;

    fn validate(&self, payment_method: PaymentMethodType) -> Result<(), Self::Rejection> {
        if payment_method == PaymentMethodType::Card {
            return Ok(());
        }

        Err(MethodPolicyRejection::UnsupportedMethod(payment_method))
    }
}

#[test]
fn external_payment_method_policy_can_accept_a_method() {
    let policy = CardOnlyPolicy;

    assert_eq!(policy.validate(PaymentMethodType::Card), Ok(()));
}

#[test]
fn external_payment_method_policy_preserves_typed_rejection() {
    let policy = CardOnlyPolicy;

    assert_eq!(
        policy.validate(PaymentMethodType::Upi),
        Err(MethodPolicyRejection::UnsupportedMethod(
            PaymentMethodType::Upi
        ))
    );
}

#[test]
fn policy_evaluation_does_not_require_a_payment() {
    let policy = CardOnlyPolicy;

    assert_eq!(
        policy.validate(PaymentMethodType::BankTransfer),
        Err(MethodPolicyRejection::UnsupportedMethod(
            PaymentMethodType::BankTransfer
        ))
    );
}
