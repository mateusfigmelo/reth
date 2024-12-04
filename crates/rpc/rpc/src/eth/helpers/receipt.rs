//! Builds an RPC receipt response w.r.t. data layout of network.

use reth_primitives::{Receipt, TransactionMeta, TransactionSigned};
use reth_provider::TransactionsProvider;
use reth_rpc_eth_api::{helpers::LoadReceipt, FromEthApiError, RpcNodeCoreExt, RpcReceipt};
use reth_rpc_eth_types::{EthApiError, EthReceiptBuilder};

use crate::EthApi;

impl<Provider, Pool, Network, EvmConfig> LoadReceipt for EthApi<Provider, Pool, Network, EvmConfig>
where
    Self: RpcNodeCoreExt<Provider: TransactionsProvider<Transaction = TransactionSigned>>,
{
    async fn build_transaction_receipt(
        &self,
        tx: TransactionSigned,
        meta: TransactionMeta,
        receipt: Receipt,
    ) -> Result<RpcReceipt<Self::NetworkTypes>, Self::Error> {
        // let start_time = std::time::Instant::now();
        let hash = meta.block_hash;
        // get all receipts for the block
        let all_receipts = self
            .cache()
            .get_receipts(hash)
            .await
            .map_err(Self::Error::from_eth_err)?
            .ok_or(EthApiError::HeaderNotFound(hash.into()))?;
        // let duration = start_time.elapsed();
        // debug!(target:"rpc_eth_receipt", build_transaction_receipt = duration.as_secs_f64(), "time duration: ");
        Ok(EthReceiptBuilder::new(&tx, meta, &receipt, &all_receipts)?.build())
    }
}
