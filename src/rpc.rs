use std::{collections::HashMap, net::SocketAddr};

use bigdecimal::BigDecimal;
use jsonrpsee::{
    core::async_trait,
    http_client::HttpClient,
    proc_macros::rpc,
    server::{ServerBuilder, ServerHandle},
};
use zksync_types::{
    api::{
        BlockDetails, BridgeAddresses, L1BatchDetails, L2ToL1LogProof, Proof, ProtocolVersion,
        TransactionDetails,
    },
    fee_model::FeeParams,
    transaction_request::CallRequest,
    Address, L1BatchNumber, MiniblockNumber, H256, U256, U64,
};
use zksync_web3_decl::types::Token;

use crate::ClaraError;

#[derive(Clone, Debug)]
pub struct Server {
    pub zksync: HttpClient,
}

impl Server {
    pub fn new(zksync: HttpClient) -> Server {
        Server { zksync }
    }

    pub async fn run(self) -> anyhow::Result<(SocketAddr, ServerHandle)> {
        let server = ServerBuilder::default()
            .build("127.0.0.1:7000")
            .await
            .unwrap();

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}

#[rpc(server, client, namespace = "zks")]
pub trait ZksApi {
    #[method(name = "estimateGasL1ToL2")]
    async fn estimate_gas_l1_to_l2(&self, req: CallRequest) -> Result<U256, ClaraError>;

    #[method(name = "getMainContract")]
    async fn get_main_contract(&self) -> Result<Address, ClaraError>;

    #[method(name = "getTestnetPaymaster")]
    async fn get_testnet_paymaster(&self) -> Result<Option<Address>, ClaraError>;

    #[method(name = "getBridgeContracts")]
    async fn get_bridge_contracts(&self) -> Result<BridgeAddresses, ClaraError>;

    #[method(name = "L1ChainId")]
    async fn l1_chain_id(&self) -> Result<U64, ClaraError>;

    #[method(name = "getConfirmedTokens")]
    async fn get_confirmed_tokens(&self, from: u32, limit: u8) -> Result<Vec<Token>, ClaraError>;

    #[method(name = "getTokenPrice")]
    async fn get_token_price(&self, token_address: Address) -> Result<BigDecimal, ClaraError>;

    #[method(name = "getAllAccountBalances")]
    async fn get_all_account_balances(
        &self,
        address: Address,
    ) -> Result<HashMap<Address, U256>, ClaraError>;

    #[method(name = "getL2ToL1MsgProof")]
    async fn get_l2_to_l1_msg_proof(
        &self,
        block: MiniblockNumber,
        sender: Address,
        msg: H256,
        l2_log_position: Option<usize>,
    ) -> Result<Option<L2ToL1LogProof>, ClaraError>;

    #[method(name = "getL2ToL1LogProof")]
    async fn get_l2_to_l1_log_proof(
        &self,
        tx_hash: H256,
        index: Option<usize>,
    ) -> Result<Option<L2ToL1LogProof>, ClaraError>;

    #[method(name = "L1BatchNumber")]
    async fn get_l1_batch_number(&self) -> Result<U64, ClaraError>;

    #[method(name = "getL1BatchBlockRange")]
    async fn get_miniblock_range(
        &self,
        batch: L1BatchNumber,
    ) -> Result<Option<(U64, U64)>, ClaraError>;

    #[method(name = "getBlockDetails")]
    async fn get_block_details(
        &self,
        block_number: MiniblockNumber,
    ) -> Result<Option<BlockDetails>, ClaraError>;

    #[method(name = "getTransactionDetails")]
    async fn get_transaction_details(
        &self,
        hash: H256,
    ) -> Result<Option<TransactionDetails>, ClaraError>;

    #[method(name = "getRawBlockTransactions")]
    async fn get_raw_block_transactions(
        &self,
        block_number: MiniblockNumber,
    ) -> Result<Vec<zksync_types::Transaction>, ClaraError>;

    #[method(name = "getL1BatchDetails")]
    async fn get_l1_batch_details(
        &self,
        batch: L1BatchNumber,
    ) -> Result<Option<L1BatchDetails>, ClaraError>;

    #[method(name = "getBytecodeByHash")]
    async fn get_bytecode_by_hash(&self, hash: H256) -> Result<Option<Vec<u8>>, ClaraError>;

    #[method(name = "getL1GasPrice")]
    async fn get_l1_gas_price(&self) -> Result<U64, ClaraError>;

    #[method(name = "getFeeParams")]
    async fn get_fee_params(&self) -> Result<FeeParams, ClaraError>;

    #[method(name = "getProtocolVersion")]
    async fn get_protocol_version(
        &self,
        version_id: Option<u16>,
    ) -> Result<Option<ProtocolVersion>, ClaraError>;

    #[method(name = "getProof")]
    async fn get_proof(
        &self,
        address: Address,
        keys: Vec<H256>,
        l1_batch_number: L1BatchNumber,
    ) -> Result<Proof, ClaraError>;
}

#[async_trait]
impl ZksApiServer for Server {
    async fn estimate_gas_l1_to_l2(&self, req: CallRequest) -> Result<U256, ClaraError> {
        self.zksync
            .estimate_gas_l1_to_l2(req)
            .await
            .map_err(Into::into)
    }

    async fn get_main_contract(&self) -> Result<Address, ClaraError> {
        self.zksync.get_main_contract().await.map_err(Into::into)
    }

    async fn get_testnet_paymaster(&self) -> Result<Option<Address>, ClaraError> {
        self.zksync
            .get_testnet_paymaster()
            .await
            .map_err(Into::into)
    }

    async fn get_bridge_contracts(&self) -> Result<BridgeAddresses, ClaraError> {
        self.zksync.get_bridge_contracts().await.map_err(Into::into)
    }

    async fn l1_chain_id(&self) -> Result<U64, ClaraError> {
        self.zksync.l1_chain_id().await.map_err(Into::into)
    }

    async fn get_confirmed_tokens(&self, from: u32, limit: u8) -> Result<Vec<Token>, ClaraError> {
        self.zksync
            .get_confirmed_tokens(from, limit)
            .await
            .map_err(Into::into)
    }

    async fn get_token_price(&self, token_address: Address) -> Result<BigDecimal, ClaraError> {
        self.zksync
            .get_token_price(token_address)
            .await
            .map_err(Into::into)
    }

    async fn get_all_account_balances(
        &self,
        address: Address,
    ) -> Result<HashMap<Address, U256>, ClaraError> {
        self.zksync
            .get_all_account_balances(address)
            .await
            .map_err(Into::into)
    }

    async fn get_l2_to_l1_msg_proof(
        &self,
        block: MiniblockNumber,
        sender: Address,
        msg: H256,
        l2_log_position: Option<usize>,
    ) -> Result<Option<L2ToL1LogProof>, ClaraError> {
        self.zksync
            .get_l2_to_l1_msg_proof(block, sender, msg, l2_log_position)
            .await
            .map_err(Into::into)
    }

    async fn get_l2_to_l1_log_proof(
        &self,
        tx_hash: H256,
        index: Option<usize>,
    ) -> Result<Option<L2ToL1LogProof>, ClaraError> {
        self.zksync
            .get_l2_to_l1_log_proof(tx_hash, index)
            .await
            .map_err(Into::into)
    }

    async fn get_l1_batch_number(&self) -> Result<U64, ClaraError> {
        self.zksync.get_l1_batch_number().await.map_err(Into::into)
    }

    async fn get_miniblock_range(
        &self,
        batch: L1BatchNumber,
    ) -> Result<Option<(U64, U64)>, ClaraError> {
        self.zksync
            .get_miniblock_range(batch)
            .await
            .map_err(Into::into)
    }

    async fn get_block_details(
        &self,
        block_number: MiniblockNumber,
    ) -> Result<Option<BlockDetails>, ClaraError> {
        self.zksync
            .get_block_details(block_number)
            .await
            .map_err(Into::into)
    }

    async fn get_transaction_details(
        &self,
        hash: H256,
    ) -> Result<Option<TransactionDetails>, ClaraError> {
        self.zksync
            .get_transaction_details(hash)
            .await
            .map_err(Into::into)
    }

    async fn get_raw_block_transactions(
        &self,
        block_number: MiniblockNumber,
    ) -> Result<Vec<zksync_types::Transaction>, ClaraError> {
        self.zksync
            .get_raw_block_transactions(block_number)
            .await
            .map_err(Into::into)
    }

    async fn get_l1_batch_details(
        &self,
        batch: L1BatchNumber,
    ) -> Result<Option<L1BatchDetails>, ClaraError> {
        self.zksync
            .get_l1_batch_details(batch)
            .await
            .map_err(Into::into)
    }

    async fn get_bytecode_by_hash(&self, hash: H256) -> Result<Option<Vec<u8>>, ClaraError> {
        self.zksync
            .get_bytecode_by_hash(hash)
            .await
            .map_err(Into::into)
    }

    async fn get_l1_gas_price(&self) -> Result<U64, ClaraError> {
        self.zksync.get_l1_gas_price().await.map_err(Into::into)
    }

    async fn get_fee_params(&self) -> Result<FeeParams, ClaraError> {
        self.zksync.get_fee_params().await.map_err(Into::into)
    }

    async fn get_protocol_version(
        &self,
        version_id: Option<u16>,
    ) -> Result<Option<ProtocolVersion>, ClaraError> {
        self.zksync
            .get_protocol_version(version_id)
            .await
            .map_err(Into::into)
    }

    async fn get_proof(
        &self,
        address: Address,
        keys: Vec<H256>,
        l1_batch_number: L1BatchNumber,
    ) -> Result<Proof, ClaraError> {
        self.zksync
            .get_proof(address, keys, l1_batch_number)
            .await
            .map_err(Into::into)
    }
}
