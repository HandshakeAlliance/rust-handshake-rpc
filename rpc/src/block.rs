use crate::client::HandshakeRpcClient;
use crate::Result;
use serde_json::json;
use handshake_client_types::{GetBlockchainInfo, GetBlock, GetBlockHeader};
use extended_primitives::Hash;

impl HandshakeRpcClient {
    pub async fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
        self.call("getblockchaininfo", &[]).await
    }

    pub async fn get_best_block_hash(&self) -> Result<Hash> {
        self.call("getbestblockhash", &[]).await
    }

    pub async fn get_block_count(&self) -> Result<u32> {
        self.call("getblockcount", &[]).await
    }

    /// get_block returns the block in Hex format.
    pub async fn get_block(
        &self,
        blockhash: &str,
    ) -> Result<String> {
        let params = vec![json!(blockhash), json!(false), json!(false)];

        self.call("getblock", &params).await
    }

    pub async fn get_block_verbose(
        &self,
        blockhash: &str,
        details: bool,
    ) -> Result<GetBlock> {
        let params = vec![json!(blockhash), json!(true), json!(details)];

        self.call("getblock", &params).await
    }

    //Returns a hex of the block
    pub async fn get_block_by_height(
        &self,
        blockheight: u32,
    ) -> Result<String> {
        let params = vec![json!(blockheight), json!(false), json!(false)];

        self.call("getblockbyheight", &params).await
    }

    pub async fn get_block_by_height_verbose(
        &self,
        blockheight: u32,
        details: bool,
    ) -> Result<GetBlock> {
        let params = vec![json!(blockheight), json!(true), json!(details)];

        self.call("getblockbyheight", &params).await
    }

    pub async fn get_block_hash(&self, height: u32) -> Result<Hash> {
        let params = vec![json!(height)];
         self.call("getblockhash", &params).await
     }

    //Returns a hex of the block header
    pub async fn get_block_header(
        &self,
        blockhash: &str,
    ) -> Result<String> {
        let params = vec![json!(blockhash), json!(false)];
        self.call("getblockheader", &params).await

    }
    pub async fn get_block_header_verbose(
        &self,
        blockhash: &str,
    ) -> Result<GetBlockHeader> {
        let params = vec![json!(blockhash), json!(true)];
        self.call("getblockheader", &params).await
    }

}