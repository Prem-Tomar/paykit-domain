# paykit-domain

Payment workflow domain types built on top of `paykit-money`.

This crate owns payment lifecycle vocabulary and checked state transitions. It does not own
transport, persistence, processor adapters, ledger posting, or network behavior.

## Current Scope

- `PaymentId`
- `PaymentStatus`
- `PaymentOperation`
- `Payment`
- checked authorization, capture, cancellation, and void transitions

```rust
use paykit_domain::{Payment, PaymentId, PaymentStatus};
use paykit_money::{Currency, Money, PaymentAmount};

let usd = Currency::new("USD", 2)?;
let amount = PaymentAmount::new(Money::from_minor_units(1_000, usd))?;
let id = PaymentId::new("pay_001")?;
let mut payment = Payment::new(id, amount);

assert_eq!(payment.status(), PaymentStatus::Created);
payment.authorize()?;
assert_eq!(payment.status(), PaymentStatus::Authorized);

# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
