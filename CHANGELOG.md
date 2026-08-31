# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Added

- Validated caller-provided payment identifiers.
- Payment lifecycle states for creation, authorization, capture, cancellation, and voiding.
- Checked lifecycle operations with typed invalid-transition errors.
- Explicit `PaymentAction` and `PaymentActionResult` vocabulary distinguishing a requested action
  from its successful in-memory transition evidence.
- Successful lifecycle action results containing the action and previous and resulting statuses.
- Read-only query helpers for inspecting payment action results.
- Payment values that preserve validated amounts from `paykit-money`.
- Regression coverage for construction, valid transitions, terminal states, error reporting, and
  custom currencies.
- Cargo package metadata, a versioned local `paykit-money` dependency contract, and Apache
  License 2.0 licensing.

### Release Status

- Version `0.1.0` is under development and has not been published.
- Package verification requires the compatible `paykit-money` version to be available from the
  selected registry.
