#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct EventFactory;

#[contractimpl]
impl EventFactory {
    pub fn hello(env: Env) -> String {
        "EventProof initialized successfully!".into()
    }
}
