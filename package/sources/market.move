module capsule_baby::market {
    use std::vector;
    use std::ascii::String;
    use std::type_name;

    use sui::transfer;
    use sui::object::{Self, ID, UID};
    use sui::tx_context::{Self, TxContext};

    use ownership::tx_authority;

    use transfer_system::royalty_market::{Self, Royalty};
    use transfer_system::market_account::{Self, MarketAccount};

    use capsule_baby::capsule_baby::{Self, CapsuleBaby};

    struct Offer has copy, store, drop {
        price: u64,
        item_id: ID,
        user: address,
        coin_type: String,
    }

    struct Registry has key {
        id: UID,
        buy_offers: vector<Offer>,
        sell_offers: vector<Offer>,
    }

    fun init(ctx: &mut TxContext) {
        let registry = Registry {
            id: object::new(ctx),
            buy_offers: vector::empty(),
            sell_offers: vector::empty(),
        };

        transfer::share_object(registry)
    }

    public entry fun create_account(ctx: &mut TxContext) {
        market_account::create(ctx)
    }

    public entry fun create_sell_offer<C>(
        registry: &mut Registry,
        capsule_baby: &mut CapsuleBaby,
        royalty: &Royalty<CapsuleBaby>,
        price: u64,
        ctx: &mut TxContext
    ) {
        let uid = capsule_baby::extend(capsule_baby);
        let sender = tx_context::sender(ctx);
        let auth = tx_authority::begin(ctx);

        royalty_market::create_sell_offer<CapsuleBaby, C>(uid, royalty, sender, price, &auth);

        let offer = Offer {
            price,
            user: sender,
            item_id: object::id(capsule_baby),
            coin_type: type_name::into_string(type_name::get<C>())
        };
        vector::push_back(&mut registry.sell_offers, offer);
    }

    public entry fun create_buy_offer<C>(
        registry: &mut Registry,
        capsule_baby: &mut CapsuleBaby,
        account: &mut MarketAccount,
        royalty: &Royalty<CapsuleBaby>,
        price: u64,
        ctx: &mut TxContext
    ) {
        let uid = capsule_baby::extend(capsule_baby);
        let sender = tx_context::sender(ctx);
        let auth = tx_authority::begin(ctx);

        royalty_market::create_buy_offer<CapsuleBaby, C>(uid, account, royalty, sender, price, &auth);

        let offer = Offer {
            price,
            user: sender,
            item_id: object::id(capsule_baby),
            coin_type: type_name::into_string(type_name::get<C>())
        };
        vector::push_back(&mut registry.buy_offers, offer);
    }
}