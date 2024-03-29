// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot Client
//!
//! Provides the [`AbstractClient`] trait that is a super trait that combines all the traits the client implements.
//! There is also the [`Client`] enum that combines all the different clients into one common structure.

use polkadot_primitives::{
	runtime_api::ParachainHost,
	v2::{AccountId, Balance, Block, BlockNumber,  Nonce},
};
use sc_client_api::{  UsageProvider};
use sc_executor::NativeElseWasmExecutor;
use sp_api::{Encode};
use sp_runtime::{
	traits::{BlakeTwo256, Block as BlockT},
};
use std::sync::Arc;
use relay_mvp_net_selection::relay_mvp_runtime;

pub mod benchmarking;

pub type FullBackend = sc_service::TFullBackend<Block>;

pub type FullClient<RuntimeApi, ExecutorDispatch> =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;

/// The native executor instance for Westend.
pub struct RelayExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for RelayExecutorDispatch {
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		relay_mvp_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		relay_mvp_runtime::native_version()
	}
}
 
/// A set of APIs that polkadot-like runtimes must implement.
pub trait RuntimeApiCollection:
	sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ sp_api::ApiExt<Block>
	+ sp_consensus_babe::BabeApi<Block>
	+ sp_finality_grandpa::GrandpaApi<Block>
	+ ParachainHost<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	+ sp_mmr_primitives::MmrApi<Block, <Block as BlockT>::Hash, BlockNumber>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
	+ sp_api::Metadata<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
	+ beefy_primitives::BeefyApi<Block>
where
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
	Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::ApiExt<Block>
		+ sp_consensus_babe::BabeApi<Block>
		+ sp_finality_grandpa::GrandpaApi<Block>
		+ ParachainHost<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
		+ sp_mmr_primitives::MmrApi<Block, <Block as BlockT>::Hash, BlockNumber>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
		+ sp_api::Metadata<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
		+ beefy_primitives::BeefyApi<Block>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}


/* 
pub trait AbstractClient<Block, Backend>:
	BlockchainEvents<Block>
	+ Sized
	+ Send
	+ Sync
	+ ProvideRuntimeApi<Block>
	+ HeaderBackend<Block>
	+ CallApiAt<Block, StateBackend = Backend::State>
	+ AuxStore
	+ UsageProvider<Block>
	+ HeaderMetadata<Block, Error = sp_blockchain::Error>
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Self::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}

impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Client: BlockchainEvents<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ UsageProvider<Block>
		+ Sized
		+ Send
		+ Sync
		+ CallApiAt<Block, StateBackend = Backend::State>
		+ HeaderMetadata<Block, Error = sp_blockchain::Error>,
	Client::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}
*/

/* 
pub trait ExecuteWithClient {
	/// The return type when calling this instance.
	type Output;

	/// Execute whatever should be executed with the given client instance.
	fn execute_with_client<Client, Api, Backend>(self, client: Arc<Client>) -> Self::Output
	where
		<Api as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
		Backend: sc_client_api::Backend<Block> + 'static,
		Backend::State: sp_api::StateBackend<BlakeTwo256>,
		Api: crate::RuntimeApiCollection<StateBackend = Backend::State>,
		Client: AbstractClient<Block, Backend, Api = Api> + 'static;
}

pub trait ClientHandle {
	/// Execute the given something with the client.
	fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output;
}
*/

/* 
macro_rules! with_client {
	{
		// The client instance that should be unwrapped.
		$self:expr,
		// The name that the unwrapped client will have.
		$client:ident,
		// NOTE: Using an expression here is fine since blocks are also expressions.
		$code:expr
	} => {
		match $self {
			Client::Westend($client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;

				$code
			},
		}
	}
}
*/
// Make the macro available only within this crate.
//pub(crate) use with_client;

/* 
#[derive(Clone)]
pub enum Client {
	Westend(Arc<FullClient<relay_mvp_runtime::RuntimeApi, RelayExecutorDispatch>>),
}
*/

/* 
impl ClientHandle for Client {
    fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output {
        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
                { T::execute_with_client::<_, _, FullBackend>(t, client.clone()) }
            }
        }
    }
}

impl UsageProvider<Block> for Client {
	fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.usage_info()
				}
			}
        }

	}
}

impl sc_client_api::BlockBackend<Block> for Client {
	fn block_body(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.block_body(hash)
				}
			}
        }
	}

	fn block(&self, hash: <Block as BlockT>::Hash) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.block(hash)
				}

			}
        }

	}


	fn block_status(&self, hash: <Block as BlockT>::Hash) -> sp_blockchain::Result<BlockStatus> {
		match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.block_status(hash)
				}
	
			}
        }
	}


	fn justifications(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Justifications>> {
        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.justifications(hash)
				}
		
			}
        }
	}

	fn block_hash(
		&self,
		number: NumberFor<Block>,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.block_hash(number)
				}
			
			}
        }

	}

	fn indexed_transaction(
		&self,
		id: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<u8>>> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.indexed_transaction(id)
				}
		
			}
        }

	}

	fn block_indexed_body(
		&self,
		id: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {

        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.block_indexed_body(id)
				}
				
			}
        }

	}

	fn requires_full_sync(&self) -> bool {


        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.requires_full_sync()
				}
			
			}
        }

	}
}

impl sc_client_api::StorageProvider<Block, crate::FullBackend> for Client {
	fn storage(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {


        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.storage(hash, key)
				}
				
			}
        }
	}

	fn storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<StorageKey>> {
        match self {
            Client::Westend(client) => {
                #[allow(unused_imports)]
                use relay_mvp_runtime as runtime;
				{
					client.storage_keys(hash, key_prefix)
				}
					
			}
        }
	}

	fn storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.storage_hash(hash, key)
				}
			}
		}
	}

	fn storage_pairs(
		&self,
		hash: <Block as BlockT>::Hash,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<(StorageKey, StorageData)>> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.storage_pairs(hash, key_prefix)
				}
					
			}
		}
	}

	fn storage_keys_iter<'a>(
		&self,
		hash: <Block as BlockT>::Hash,
		prefix: Option<&'a StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<
		KeyIterator<'a, <crate::FullBackend as sc_client_api::Backend<Block>>::State, Block>,
	> {
        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.storage_keys_iter(hash, prefix, start_key)
				}
					
			}
		}
	}

	fn child_storage(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.child_storage(hash, child_info, key)
				}
					
			}
		}

	}

	fn child_storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &ChildInfo,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<StorageKey>> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.child_storage_keys(hash, child_info, key_prefix)
				}
					
			}
		}
	}

	fn child_storage_keys_iter<'a>(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: ChildInfo,
		prefix: Option<&'a StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<
		KeyIterator<'a, <crate::FullBackend as sc_client_api::Backend<Block>>::State, Block>,
	> {
        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.child_storage_keys_iter(hash, child_info, prefix, start_key)
				}
					
			}
		}

	}

	fn child_storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.child_storage_hash(hash, child_info, key)
				}
					
			}
		}
	}
}

impl sp_blockchain::HeaderBackend<Block> for Client {
	fn header(&self, hash: Hash) -> sp_blockchain::Result<Option<Header>> {

		match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.header(hash)
				}
					
			}
		}
	}



	fn info(&self) -> sp_blockchain::Info<Block> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.info()
				}
			}
		}


	}





	fn status(&self, hash: Hash) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {

        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.status(hash)
				}
			}
		}
	}

	fn number(&self, hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
        match self {

			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.number(hash)
				}
			}
		}

	}

	fn hash(&self, number: BlockNumber) -> sp_blockchain::Result<Option<Hash>> {
        match self {
			Client::Westend(client) => {
				#[allow(unused_imports)]
				use relay_mvp_runtime as runtime;
				{
					client.hash(number)
				}
			}
		}
	}
}
*/