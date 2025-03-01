#![cfg_attr(not(feature = "std"), no_std)]

// Export the pallet to make it accessible from the runtime.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use serde::{Deserialize, Serialize};

    // Structure to hold metadata for a subnet.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, Serialize, Deserialize)]
    pub struct SubnetMetadata {
        title: Vec<u8>,               // Title of the subnet
        intro: Vec<u8>,               // Introduction or description of the subnet
        rewards_allocation: u32,      // Rewards allocation for the subnet
        core_performance: u32,        // Core performance metric
        gpunet_performance: u32,      // GPU network performance metric
        metadata: Vec<u8>,            // Additional metadata
    }

    // Structure to hold metadata for a provider.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, Serialize, Deserialize)]
    pub struct ProviderMetadata {
        name: Vec<u8>,                // Name of the provider
        resource_details: Vec<u8>,    // Details about the resources provided
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // Event type used by the pallet.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Storage map to store subnets, keyed by the account ID of the subnet owner.
    #[pallet::storage]
    #[pallet::getter(fn subnets)]
    pub type Subnets<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, SubnetMetadata, OptionQuery>;

    // Storage map to store providers, keyed by the account ID of the subnet owner.
    #[pallet::storage]
    #[pallet::getter(fn providers)]
    pub type Providers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<ProviderMetadata>, ValueQuery>;

    // Events emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Event emitted when a subnet is created.
        SubnetCreated(T::AccountId, SubnetMetadata),
        // Event emitted when a provider is registered.
        ProviderRegistered(T::AccountId, ProviderMetadata),
    }

    // Errors that can occur within the pallet.
    #[pallet::error]
    pub enum Error<T> {
        SubnetAlreadyExists,          // Error when a subnet already exists for the account
        ProviderAlreadyRegistered,    // Error when a provider is already registered
        SubnetNotFound,               // Error when the specified subnet is not found
    }

    // Dispatchable functions (extrinsics) for the pallet.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Function to create a new subnet.
        #[pallet::weight(10_000)]
        pub fn create_subnet(
            origin: OriginFor<T>,
            metadata: SubnetMetadata,
        ) -> DispatchResult {
            // Ensure the caller is a signed account.
            let who = ensure_signed(origin)?;

            // Ensure the subnet does not already exist for the caller.
            ensure!(!Subnets::<T>::contains_key(&who), Error::<T>::SubnetAlreadyExists);

            // Insert the subnet metadata into storage.
            Subnets::<T>::insert(&who, metadata.clone());

            // Emit an event indicating the subnet was created.
            Self::deposit_event(Event::SubnetCreated(who, metadata));
            Ok(())
        }

        // Function to register a provider for a subnet.
        #[pallet::weight(10_000)]
        pub fn register_provider(
            origin: OriginFor<T>,
            subnet_owner: T::AccountId,
            provider_metadata: ProviderMetadata,
        ) -> DispatchResult {
            // Ensure the caller is a signed account.
            let who = ensure_signed(origin)?;

            // Ensure the specified subnet exists.
            ensure!(Subnets::<T>::contains_key(&subnet_owner), Error::<T>::SubnetNotFound);

            // Add the provider metadata to the list of providers for the subnet.
            Providers::<T>::mutate(&subnet_owner, |providers| {
                providers.push(provider_metadata.clone());
            });

            // Emit an event indicating the provider was registered.
            Self::deposit_event(Event::ProviderRegistered(who, provider_metadata));
            Ok(())
        }
    }
}