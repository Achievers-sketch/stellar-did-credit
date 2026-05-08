#[cfg(test)]
mod tests {
    use credit_oracle::{CreditOracle, CreditOracleClient, TxStats};
    use identity_oracle::{IdentityOracle, IdentityOracleClient};
    use revocation_registry::{RevocationRegistry, RevocationRegistryClient};
    use soroban_sdk::{testutils::Address as _, BytesN, Env, String};

    #[test]
    fn test_full_protocol_flow() {
        // 1. Create Env with mock_all_auths
        let env = Env::default();
        env.mock_all_auths();

        // 2. Register and initialize all 3 contracts
        let identity_id = env.register_contract(None, IdentityOracle);
        let credit_id = env.register_contract(None, CreditOracle);
        let _revocation_id = env.register_contract(None, RevocationRegistry);

        let identity = IdentityOracleClient::new(&env, &identity_id);
        let credit = CreditOracleClient::new(&env, &credit_id);
        let revocation = RevocationRegistryClient::new(&env, &_revocation_id);

        let admin = soroban_sdk::Address::generate(&env);
        identity.initialize(&admin);
        credit.initialize(&admin);
        revocation.initialize(&admin);

        // 3. Register an issuer in identity-oracle
        let issuer = soroban_sdk::Address::generate(&env);
        identity.register_issuer(&admin, &issuer);

        // 4. Call anchor_did for a test subject
        let subject = soroban_sdk::Address::generate(&env);
        let cid = String::from_str(&env, "ipfs://QmTestDID");
        identity.anchor_did(&subject, &cid);

        // 5. Call anchor_vc for the subject with a test hash
        let vc_hash = BytesN::from_array(&env, &[42u8; 32]);
        identity.anchor_vc(&issuer, &subject, &vc_hash);

        // 6. Assert is_verified returns true
        assert!(identity.is_verified(&subject));

        // 7. Register a lender and feeder in credit-oracle
        let lender = soroban_sdk::Address::generate(&env);
        let feeder = soroban_sdk::Address::generate(&env);
        credit.register_lender(&admin, &lender);
        credit.register_feeder(&admin, &feeder);

        // 8. Call set_vc_count(subject, 1)
        credit.set_vc_count(&feeder, &subject, &1);

        // 9. Call update_tx_stats with volume_30d = 500_000_000 stroops
        credit.update_tx_stats(
            &feeder,
            &subject,
            &TxStats {
                volume_30d: 500_000_000i128,
                tx_count_30d: 10,
                avg_counterparties: 3,
            },
        );

        // 10. Call record_repayment 5 times on_time=true
        for _ in 0..5 {
            credit.record_repayment(&lender, &subject, &100_000_000i128, &true);
        }

        // 11. Call compute_score
        let score = credit.compute_score(&subject);

        // 12. Assert score > 300
        assert!(score > 300, "expected score > 300, got {}", score);

        // 13. Assert score <= 850
        assert!(score <= 850, "expected score <= 850, got {}", score);
    }
}
