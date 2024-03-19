// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::jsonrpc_types::address::BitcoinAddressView;
use crate::jsonrpc_types::btc::transaction::{hex_to_txid, TxidView};
use crate::jsonrpc_types::{AccountAddressView, StructTagView};
use anyhow::Result;
use bitcoin::hashes::Hash;
use bitcoin::Txid;
use move_core_types::account_address::AccountAddress;
use moveos_types::moveos_std::object;
use moveos_types::moveos_std::object::ObjectID;
use moveos_types::state::MoveStructType;
use rooch_types::bitcoin::utxo::{BitcoinOutputID, OutputID, UTXOState, UTXO};
use rooch_types::indexer::state::GlobalStateFilter;
use rooch_types::into_address::IntoAddress;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, JsonSchema)]
pub struct BitcoinOutputIDView {
    // pub txid: AccountAddress,
    pub txid: TxidView,
    pub vout: u32,
}

impl From<BitcoinOutputIDView> for BitcoinOutputID {
    fn from(inscription: BitcoinOutputIDView) -> Self {
        BitcoinOutputID {
            txid: inscription.txid.into(),
            vout: inscription.vout,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UTXOFilterView {
    /// Query by owner, represent by bitcoin address
    Owner(BitcoinAddressView),
    /// Query by bitcoin output id, represent by bitcoin txid and vout
    OutputId { txid: String, vout: u32 },
    /// Query by object id.
    ObjectId(ObjectID),
    /// Query all.
    All,
}

impl UTXOFilterView {
    pub fn into_global_state_filter(
        filter_opt: UTXOFilterView,
        resolve_address: AccountAddress,
    ) -> Result<GlobalStateFilter> {
        Ok(match filter_opt {
            UTXOFilterView::Owner(_owner) => GlobalStateFilter::ObjectTypeWithOwner {
                object_type: UTXO::struct_tag(),
                owner: resolve_address,
            },
            UTXOFilterView::OutputId { txid, vout } => {
                let txid = hex_to_txid(txid.as_str())?;
                let output_id = OutputID::new(txid.into_address(), vout);
                let object_id = object::custom_object_id(output_id, &UTXO::struct_tag());

                GlobalStateFilter::ObjectId(object_id)
            }
            UTXOFilterView::ObjectId(object_id) => GlobalStateFilter::ObjectId(object_id),
            UTXOFilterView::All => GlobalStateFilter::ObjectType(UTXO::struct_tag()),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UTXOView {
    /// The txid of the UTXO
    txid: AccountAddressView,
    /// The txid of the UTXO
    bitcoin_txid: TxidView,
    /// The vout of the UTXO
    vout: u32,
    /// The value of the UTXO
    value: u64,
    /// Protocol seals
    seals: String,
}

impl UTXOView {
    pub fn try_new_from_utxo(utxo: UTXO) -> Result<UTXOView, anyhow::Error> {
        let bitcoin_txid = Txid::from_byte_array(utxo.txid.into_bytes());
        // reversed bytes of txid
        let seals_str = serde_json::to_string(&utxo.seals)?;

        Ok(UTXOView {
            txid: utxo.txid.into(),
            bitcoin_txid: bitcoin_txid.into(),
            vout: utxo.vout,
            value: utxo.value,
            seals: seals_str,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UTXOStateView {
    pub object_id: ObjectID,
    pub owner: AccountAddressView,
    pub owner_bitcoin_address: Option<String>,
    pub flag: u8,
    pub value: Option<UTXOView>,
    pub object_type: StructTagView,
    pub tx_order: u64,
    pub state_index: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl UTXOStateView {
    pub fn try_new_from_utxo_state(
        utxo: UTXOState,
        network: u8,
    ) -> Result<UTXOStateView, anyhow::Error> {
        let owner_bitcoin_address = match utxo.owner_bitcoin_address {
            Some(baddress) => Some(baddress.format(network)?),
            None => None,
        };
        let utxo_view = match utxo.value {
            Some(utxo) => Some(UTXOView::try_new_from_utxo(utxo)?),
            None => None,
        };
        Ok(UTXOStateView {
            object_id: utxo.object_id,
            owner: utxo.owner.into(),
            owner_bitcoin_address,
            flag: utxo.flag,
            value: utxo_view,
            object_type: utxo.object_type.into(),
            tx_order: utxo.tx_order,
            state_index: utxo.state_index,
            created_at: utxo.created_at,
            updated_at: utxo.updated_at,
        })
    }
}
