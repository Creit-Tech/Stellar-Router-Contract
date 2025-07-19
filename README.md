# Soroban Project

A dead simple and small contract that can be used to make multiple calls at once, it's pretty useful for wallets that
need to get multiple balances simulations at once.

## Contract versions

Each version of the contract includes more features (or bug fixes) than the last one. Here we will list all of them,
even if they were removed from the front end library.

### v0: Atomic contract

This is the base contract, it allows the contract caller to execute a series of contract invocations in an atomic way
just like how it works in the classic side

### v1: Optional succeed call

This version allows the contract caller to define if they want to ignore one contract call failing. This is useful for
different use cases, an example is a service that is paying for fees while charging the user with another coin, the
service might want to only care if the fee conversion goes while the other call fails.
