# paykit-domain

Payment workflow domain types built on top of `paykit-money`.

This crate owns payment lifecycle vocabulary and checked state transitions. It does not own
transport, persistence, processor adapters, ledger posting, or network behavior.

## Current Scope

- `PaymentId`
- `PaymentStatus`
- `PaymentOperation`
- `PaymentTransition`
- `Payment`
- checked authorization, capture, cancellation, and void transitions

Successful lifecycle operations return `PaymentTransition`, which records the applied operation
and the statuses immediately before and after the in-memory state change. It is transition
evidence, not processor confirmation or a durable event.

```rust
use paykit_domain::{Payment, PaymentId, PaymentOperation, PaymentStatus};
use paykit_money::{Currency, Money, PaymentAmount};

let usd = Currency::new("USD", 2)?;
let amount = PaymentAmount::new(Money::from_minor_units(1_000, usd))?;
let id = PaymentId::new("pay_001")?;
let mut payment = Payment::new(id, amount);

assert_eq!(payment.status(), PaymentStatus::Created);
let transition = payment.authorize()?;
assert_eq!(transition.operation(), PaymentOperation::Authorize);
assert_eq!(transition.previous_status(), PaymentStatus::Created);
assert_eq!(transition.resulting_status(), PaymentStatus::Authorized);
assert_eq!(payment.status(), PaymentStatus::Authorized);

# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
