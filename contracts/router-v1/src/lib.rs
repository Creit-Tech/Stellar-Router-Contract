#![no_std]

use soroban_sdk::{contract, contractimpl, vec, Address, Env, Symbol, Val, Vec};

mod test;

#[contract]
pub struct RouterV1;

#[contractimpl]
impl RouterV1 {
    pub fn exec(
        e: Env,
        caller: Address,
        invocations: Vec<(Address, Symbol, Vec<Val>, bool)>,
    ) -> Vec<Val> {
        // This require_auth is here so we don't get the error "[recording authorization only] encountered authorization not tied to the root contract invocation for an address. Use `require_auth()` in the top invocation or enable non-root authorization."
        caller.require_auth();
        e.storage().instance().extend_ttl(17280 * 3, 17280 * 7);
        let mut results: Vec<Val> = vec![&e];
        for (contract, method, args, can_fail) in invocations {
            if can_fail {
                let result = e.try_invoke_contract::<Val, Val>(&contract, &method, args);
                match result {
                    Ok(v) => results.push_back(v.unwrap()),
                    Err(err) => results.push_back(err.unwrap()),
                }
            } else {
                results.push_back(e.invoke_contract::<Val>(&contract, &method, args));
            }
        }
        results
    }
}
