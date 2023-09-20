#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        dispatch::DispatchResultWithPostInfo, 
        pallet_prelude::*
    };
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
	pub trait Config: frame_system::Config {
        // pallet::constant 用于声明这是个链上的常量
		#[pallet::constant]
		/// The maximum length of claim that can be added.
		type MaxClaimLength: Get<u32>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(13);
    
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

    #[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor::<T>)
    >;

   	// 定义事件
	#[pallet::event]
	// 生成工具函数
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
	}

	// 定义错误
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// 用于定义回调函数，在区块的不同时期执行
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// 定义可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			// 验证签名
			let sender = ensure_signed(origin)?;

			// 尝试从 Vec<u8> 转换为 BoundedVec<u8, T::MaxClaimLength>
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			// 验证是否已经存储过
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

        #[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

        #[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&bounded_claim, (dest, frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransfered(owner, sender, claim));

			Ok(().into())
		}
	}

}
