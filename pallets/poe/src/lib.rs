#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use sp_std::prelude::Vec;
	#[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxClaimLength:Get<u32>;

	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T:Config> = StorageMap<_, 
    Blake2_128Concat,
    BoundedVec<u8,T::MaxClaimLength>,
    (T::AccountId,T::BlockNumber)
    >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(T::AccountId,BoundedVec<u8,T::MaxClaimLength> ),
		SomethingRemove(T::AccountId ,BoundedVec<u8,T::MaxClaimLength>),
        SomethingTransfer(T::AccountId,BoundedVec<u8,T::MaxClaimLength>,T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		ProofAlreadyExist,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn something_stored(origin: OriginFor<T>, something: BoundedVec<u8,T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
            ensure!(!Something::<T>::contains_key(&something),Error::<T>::ProofAlreadyExist);
            Something::<T>::insert(
                &something,
                (who.clone(),frame_system::Pallet::<T>::block_number()),
                );
			// Emit an event.
			Self::deposit_event(Event::SomethingStored(who,something));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn something_remove(origin: OriginFor<T>, something: BoundedVec<u8,T::MaxClaimLength>) -> DispatchResultWithPostInfo {

			let who = ensure_signed(origin)?;
            let (owner,_)=Something::<T>::get(&something).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner==who,Error::<T>::NotClaimOwner);
            Something::<T>::remove(&something);
			Self::deposit_event(Event::SomethingRemove(who,something ));

            Ok(().into())
        }
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
        pub fn something_transfer(origin:OriginFor<T>,hash:BoundedVec<u8,T::MaxClaimLength>,receive:T::AccountId) -> DispatchResultWithPostInfo
        {
            
			let who = ensure_signed(origin)?;
            let (owner,_)=Something::<T>::get(&hash).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner==who,Error::<T>::NotClaimOwner);
            Something::<T>::remove(&hash);
            Something::<T>::insert(&hash, (receive.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::SomethingTransfer(who,hash,receive));
            Ok(().into())
        }

	}
}
