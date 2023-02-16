use crate::contract::Contract;

#[test]
fn get_default_greeting() {
    let contract = Contract::default();
    // this test did not call set_greeting so should return the default "Hello" greeting
    assert_eq!(contract.get_greeting(), "Hello".to_string());
}

#[test]
fn set_then_get_greeting() {
    let mut contract = Contract::default();
    contract.set_greeting("howdy".to_string());
    assert_eq!(contract.get_greeting(), "howdy".to_string());
}