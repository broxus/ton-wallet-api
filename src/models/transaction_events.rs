use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct CreateSendTransactionEvent {
    pub id: Uuid,
    pub service_id: ServiceId,
    pub transaction_id: Uuid,
    pub message_hash: String,
    pub account_workchain_id: i32,
    pub account_hex: String,
    pub transaction_direction: TonTransactionDirection,
    pub transaction_status: TonTransactionStatus,
    pub event_status: TonEventStatus,
    pub balance_change: Option<BigDecimal>,
    pub multisig_transaction_id: Option<i64>,
}

impl CreateSendTransactionEvent {
    pub fn new(payload: TransactionDb) -> Self {
        Self {
            id: Uuid::new_v4(),
            service_id: payload.service_id,
            transaction_id: payload.id,
            message_hash: payload.message_hash,
            account_workchain_id: payload.account_workchain_id,
            account_hex: payload.account_hex,
            transaction_direction: payload.direction,
            transaction_status: payload.status,
            balance_change: payload.balance_change,
            multisig_transaction_id: payload.multisig_transaction_id,
            event_status: TonEventStatus::New,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct UpdateSendTransactionEvent {
    pub balance_change: Option<BigDecimal>,
    pub transaction_status: TonTransactionStatus,
    pub multisig_transaction_id: Option<i64>,
}

impl UpdateSendTransactionEvent {
    pub fn new(payload: TransactionDb) -> Self {
        Self {
            transaction_status: payload.status,
            balance_change: payload.balance_change,
            multisig_transaction_id: payload.multisig_transaction_id,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct CreateReceiveTransactionEvent {
    pub id: Uuid,
    pub service_id: ServiceId,
    pub transaction_id: Uuid,
    pub message_hash: String,
    pub account_workchain_id: i32,
    pub account_hex: String,
    pub sender_workchain_id: Option<i32>,
    pub sender_hex: Option<String>,
    pub balance_change: Option<BigDecimal>,
    pub transaction_direction: TonTransactionDirection,
    pub transaction_status: TonTransactionStatus,
    pub event_status: TonEventStatus,
}

impl CreateReceiveTransactionEvent {
    pub fn new(payload: TransactionDb) -> Self {
        Self {
            id: Uuid::new_v4(),
            service_id: payload.service_id,
            transaction_id: payload.id,
            message_hash: payload.message_hash,
            account_workchain_id: payload.account_workchain_id,
            account_hex: payload.account_hex,
            sender_workchain_id: payload.sender_workchain_id,
            sender_hex: payload.sender_hex,
            balance_change: payload.balance_change,
            transaction_direction: payload.direction,
            transaction_status: payload.status,
            event_status: TonEventStatus::New,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TransactionsEventsSearch {
    pub limit: i64,
    pub offset: i64,
    pub created_at_ge: Option<i64>,
    pub created_at_le: Option<i64>,
    pub transaction_id: Option<Uuid>,
    pub message_hash: Option<String>,
    pub account_workchain_id: Option<i32>,
    pub account_hex: Option<String>,
    pub transaction_direction: Option<TonTransactionDirection>,
    pub transaction_status: Option<TonTransactionStatus>,
    pub event_status: Option<TonEventStatus>,
}
