#![cfg_attr(not(feature = "std"), no_std)]

/// A price feed pallet
use frame_support::{debug::native, decl_error, decl_event, decl_module, decl_storage, dispatch};
use frame_system::ensure_signed;
use fetch_price::FetchPriceFor;
use orml_traits::{BasicCurrency, CurrencyId};
impl<T: Trait> FetchPrice<u32> for Module<T> {
    fn fetch_price() -> u32 {
        Self::get_price()
    }
}

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    type OffchainPrice: FetchPriceFor;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Price {
        Price get(fn get_price): u32 = 1_000_000;
        BasketPrice get(fn get_price): u32 = 1_000_000;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        CurrencyId = CurrencyIdOf<T>,
    {
        NewPrice(u32),

        DummyEvent(AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        NoOffchainPrice
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 0]
        pub fn set_price(origin, currency_id: CurrencyId, new_price: u32) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            Price::put(currency_id, new_price);

            Self::deposit_event(RawEvent::NewPrice(currency_id, new_price));

            Ok(())
        }

        #[weight = 0]
        pub fn set_basket_price(
            origin, 
            currency_id: CurrencyId,
            peg_price1: u32,
            peg_price2: u32,
            peg_price3: u32,
            peg_price4: u32,
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let new_price = (peg_price1 + peg_price2 + peg_price3 + peg_price4)/4

            BasketPrice::put(currency_id, new_price);

            Self::deposit_event(RawEvent::NewPrice(currency_id, new_price));

            Ok(())
        }

        #[weight = 0]
        pub fn get_offchain_price(origin) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let price = T::OffchainPrice::fetch_price(b"DOT").unwrap();

            native::info!("DOT offchain price: {}", price);
            Price::put(currency_id, new_price);

            Self::deposit_event(RawEvent::NewPrice(currency_id, new_price));

            Ok(())
        }
    }
}
