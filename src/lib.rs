use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Symbol, Vec,
};
#[contract]
pub struct PaymentContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentSending {
    from: Address,
    to: Address,
    amount: i128,
    message: Symbol,
    date: u64,
}
#[contractimpl]
impl PaymentContract {
    pub fn initialize(env: Env) {
        let payments: Vec<PaymentSending> = vec![&env];
        env.storage()
            .instance()
            .set(&symbol_short!("payments"), &payments);
        log!(&env, "Contract initialized");
    }
    pub fn transfer_xlm(env: Env, from: Address, to: Address, amount: i128, message: String) {
        from.require_auth();
        let transfer_event = symbol_short!("transfer");
        env.events()
            .publish((transfer_event, from.clone(), to.clone()), amount);

        if !message.is_empty() {
            let message_event = symbol_short!("message");
            env.events().publish((message_event, from, to), message);
        }
    }

    pub fn get_balance(_env: Env, _address: Address) -> i128 {
        // Gerçek uygulamada, Stellar Core'dan bakiye bilgisi alınmalı
        // Bu örnekte sabit bir değer döndürüyoruz
        1000
    }
    pub fn get_messages(env: Env, user: Address) -> Vec<String> {
        env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::new(&env))
    }
    // Düzenli ödemeler oluşturma
    pub fn create_recurring_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        interval: u64,
        message: Symbol,
    ) {
        from.require_auth();

        let recurring_payments: soroban_sdk::storage::Persistent = env.storage().persistent();
        let mut payments = recurring_payments
            .get::<_, Vec<(Address, Address, i128, u64, Symbol)>>(&symbol_short!("recurring"))
            .unwrap_or_else(|| Vec::new(&env));

        payments.push_back((from, to, amount, interval, message));
        recurring_payments.set(&symbol_short!("recurring"), &payments);
    }
    // İşlem geçmişi görüntüleme
    pub fn get_transaction_history(
        env: Env,
        account: Address,
    ) -> Vec<(Address, Address, i128, Symbol)> {
        let history = env.storage().persistent();
        history.get(&account).unwrap_or_else(|| Vec::new(&env))
    }

    // Düzenli ödemeleri işleme (bu fonksiyon düzenli olarak çağrılmalıdır)
    pub fn process_recurring_payments(env: Env) {
        let current_time = env.ledger().timestamp();
        let recurring_payments = env.storage().persistent();
        let payments = recurring_payments
            .get::<_, Vec<(Address, Address, i128, u64, Symbol)>>(&symbol_short!("recurring"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut processed_payments = Vec::new(&env);

        for (from, to, amount, interval, message) in payments.iter() {
            if current_time % interval as u64 == 0 {
                Self::transfer_xlm(
                    env.clone(),
                    from.clone(),
                    to.clone(),
                    amount,
                    message.to_string(),
                );
                processed_payments.push_back((from.clone(), to.clone(), amount, message.clone()));
            }
        }

        // İşlem geçmişini güncelle
        for (from, to, amount, message) in processed_payments.iter() {
            let history = env.storage().persistent();
            let mut account_history = history
                .get::<_, Vec<(Address, Address, i128, Symbol)>>(&from)
                .unwrap_or_else(|| Vec::new(&env));
            account_history.push_back((from.clone(), to.clone(), amount, message.clone()));
            history.set(&from, &account_history);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    use soroban_sdk::{Address, BytesN, Env};

    #[test]
    fn test_transfer() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);

        client.initialize();

        let contract_id = BytesN::from_array(&env, &[2; 32]);

        let from = Address::random(&env);
        let to = Address::random(&env);
        let amount = 100;
        let message: Symbol = symbol_short!("test");
        client.transfer_xlm(&from, &to, &amount, &message.to_string());
    }

    // #[test]
    // fn test_create_recurring_payment() {
    //     let env = Env::default();
    //     let contract_id = env.register_contract(None, PaymentContract);
    //     let client = PaymentContractClient::new(&env, &contract_id);

    //     let from = Address::random(&env);
    //     let to = Address::random(&env);
    //     let amount = 50;
    //     let interval = 86400; // 1 day in seconds
    //     let message = symbol_short!("monthly");

    //     env.mock_all_auths();

    //     env.ledger().set(LedgerInfo {
    //         timestamp: 12345,
    //         protocol_version: 20,
    //         sequence_number: 50,
    //         network_id: Default::default(),
    //         base_reserve: 10,
    //         min_temp_entry_ttl: 10,
    //         min_persistent_entry_ttl: 10,
    //         max_entry_ttl: 3110400,
    //     });

    //     client.create_recurring_payment(&from, &to, &amount, &interval, &message);

    //     assert_eq!(
    //         env.auths(),
    //         vec![(
    //             from.clone(),
    //             AuthorizedInvocation {
    //                 function: AuthorizedFunction::Contract(
    //                     contract_id.clone(),
    //                     symbol_short!("create_recurring_payment"),
    //                     (from.clone(), to.clone(), amount, interval, message).into_val(&env),
    //                 ),
    //                 sub_invocations: vec![],
    //             }
    //         )]
    //     );

    //     // Process recurring payments to trigger the payment
    //     env.ledger().set(LedgerInfo {
    //         timestamp: 12345 + interval,
    //         ..env.ledger().get()
    //     });

    //     log!(&env, "Processing recurring payments...");
    //     client.process_recurring_payments();

    //     // Check if the payment was processed
    //     let history = client.get_transaction_history(&from);
    //     log!(&env, "Transaction history: {:?}", history);

    //     assert_eq!(history.len(), 1);
    //     assert_eq!(history[0].from, from);
    //     assert_eq!(history[0].to, to);
    //     assert_eq!(history[0].amount, amount);
    //     assert_eq!(history[0].message, message);

    //     assert_eq!(client.get_balance(&to), amount);
    //     assert_eq!(client.get_balance(&from), amount);
    // }
}
