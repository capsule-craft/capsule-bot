module demo_ex::capsule_baby {
    use std::string::{Self, String};

    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;

    use sui_utils::typed_id;

    use ownership::ownership;
    use ownership::publish_receipt;
    use ownership::tx_authority;

    use transfer_system::royalty_market::{Self, Witness as RoyaltyMarket};

    struct CapsuleBaby has key {
        id: UID,
        name: String
    }

    struct Witness has drop {}
    struct CAPSULE_BABY has drop {}

    fun init(genesis: CAPSULE_BABY, ctx: &mut TxContext) {
        let royalty_bps = 1000u16;
        let marketplace_fee_bps = 200u16;
        let sender = tx_context::sender(ctx);

        let receipt = publish_receipt::claim(&genesis, ctx);

        royalty_market::create_royalty<CapsuleBaby>(&receipt, sender, royalty_bps, marketplace_fee_bps, ctx);
        transfer::public_transfer(receipt, sender)
    }

    public entry fun create(name: vector<u8>, ctx: &mut TxContext) {
        let capsule_baby = CapsuleBaby {
            id: object::new(ctx),
            name: string::utf8(name)
        };

        let owner = vector[tx_context::sender(ctx)];
        let typed_id = typed_id::new(&capsule_baby);
        let auth = tx_authority::begin_with_type(&Witness {});

        ownership::as_shared_object<CapsuleBaby, RoyaltyMarket>(&mut capsule_baby.id, typed_id, owner, &auth);

        transfer::share_object(capsule_baby)
    }
}