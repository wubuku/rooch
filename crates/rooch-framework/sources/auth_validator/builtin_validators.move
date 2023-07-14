module rooch_framework::builtin_validators{

    use moveos_std::storage_context::StorageContext;
    use rooch_framework::auth_validator_registry;
    use rooch_framework::ed25519_validator;
    use rooch_framework::multi_ed25519_validator;
    use rooch_framework::secp256k1_validator;

    friend rooch_framework::genesis;

    const E_GENESIS_INIT: u64 = 1;

    public(friend) fun genesis_init(ctx: &mut StorageContext, _genesis_account: &signer){
        //SCHEME_ED25519: u64 = 0;
        let id = auth_validator_registry::register_internal<ed25519_validator::Ed25519Validator>(ctx);
        assert!(id == ed25519_validator::scheme(), std::error::internal(E_GENESIS_INIT));
        //SCHEME_MULTIED25519: u64 = 1;
        let id = auth_validator_registry::register_internal<multi_ed25519_validator::MultiEd25519Validator>(ctx);
        assert!(id == multi_ed25519_validator::scheme(), std::error::internal(E_GENESIS_INIT));
        //SCHEME_SECP256K1: u64 = 2;
        let id = auth_validator_registry::register_internal<secp256k1_validator::Secp256k1Validator>(ctx);
        assert!(id == secp256k1_validator::scheme(), std::error::internal(E_GENESIS_INIT));
    }

    public fun is_builtin(scheme: u64): bool {
        scheme == ed25519_validator::scheme() || scheme == multi_ed25519_validator::scheme() || scheme == secp256k1_validator::scheme()
    }
}