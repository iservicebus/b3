#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchResult},
	pallet_prelude::MaxEncodedLen,
	traits::{Get},
	BoundedVec,
	DefaultNoBound, EqNoBound, PartialEqNoBound,

	pallet_prelude::{
		RuntimeDebugNoBound,
	},
};

mod tests;
mod mock;



/// A checked solution, ready to be enacted.
#[derive(
	 MaxEncodedLen,
	PartialEqNoBound,
	EqNoBound,
	Encode,
	Decode,
	RuntimeDebugNoBound,
	DefaultNoBound,
	scale_info::TypeInfo,
)]
#[scale_info(skip_type_params(T))]
pub struct LetterOfCredit<T: Config> {
	id: u64,
	trade_id: u64,
	amount: u64,
	beneficiary: BoundedVec<u8, T::StringLimit>,
	issuing_bank: BoundedVec<u8, T::StringLimit>,
	status: u64,
}

impl<T: Config> Clone for LetterOfCredit<T> {
    fn clone(&self) -> Self {
   
		LetterOfCredit {

			trade_id: self.trade_id.clone(),

			amount: self.amount.clone(),
			issuing_bank: self.issuing_bank.clone(),
			status: self.status.clone(),
			id: self.id.clone(),
			beneficiary: self.beneficiary.clone(),
		}
    }
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;





	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;


	}

	#[pallet::storage]
	#[pallet::getter(fn get_lc)]
	//  declaring LetterOfCredit as storage item:
	pub type LetterOfCreditStore<T: Config> = StorageValue<_, LetterOfCredit<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	
	pub enum Event<T: Config> {
		LetterOfCreditStored { lc: LetterOfCredit<T>, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn send_lc(origin: OriginFor<T>, lc: LetterOfCredit<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let lc_clone = lc.clone();
			// Update storage.
			<LetterOfCreditStore<T>>::put(lc_clone);

			// Emit an event.
			Self::deposit_event(Event::LetterOfCreditStored { lc, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
/* 
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn get_lc(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			 
				let lc = <LetterOfCreditStore<T>>::get();


			let lc_clone = lc.clone();
			// Update storage.
			<LetterOfCreditStore<T>>::put(lc_clone);

			// Emit an event.
			Self::deposit_event(Event::LetterOfCreditStored { lc, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

*/

/* 
		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <LetterOfCreditStore<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<LetterOfCreditStore<T>>::put(new);
					Ok(())
				},
			}
		}
*/
	}
}
