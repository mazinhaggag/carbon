use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, DatasourceId, TransactionUpdate, Update,
            UpdateType,
        },
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    futures::{sink::SinkExt, StreamExt},
    solana_account::Account,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        convert::TryFrom,
        sync::Arc,
        time::{Duration, Instant},
    },
    tokio::sync::{mpsc::{error::TrySendError, Sender}, RwLock},
    tokio_util::sync::CancellationToken,
    yellowstone_grpc_client::{GeyserGrpcBuilder, GeyserGrpcBuilderResult, GeyserGrpcClient},
    yellowstone_grpc_proto::{
        convert_from::{create_tx_meta, create_tx_versioned},
        geyser::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
            SubscribeRequestFilterAccounts, SubscribeRequestFilterBlocks,
            SubscribeRequestFilterTransactions, SubscribeRequestPing, SubscribeUpdateAccountInfo,
            SubscribeUpdateTransactionInfo,
        },
        tonic::{codec::CompressionEncoding, transport::ClientTlsConfig},
    },
};

#[derive(Debug)]
pub struct YellowstoneGrpcGeyserClient {
    pub endpoint: String,
    pub x_token: Option<String>,
    pub commitment: Option<CommitmentLevel>,
    pub account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    pub transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    pub block_filters: BlockFilters,
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    pub geyser_config: YellowstoneGrpcClientConfig,
}

#[derive(Debug, Clone)]
pub struct YellowstoneGrpcClientConfig {
    pub compression: Option<CompressionEncoding>,
    pub connect_timeout: Option<Duration>,
    pub timeout: Option<Duration>,
    pub max_decoding_message_size: Option<usize>,
    pub tls_config: Option<ClientTlsConfig>,
    pub tcp_nodelay: Option<bool>,
}

impl Default for YellowstoneGrpcClientConfig {
    fn default() -> Self {
        Self {
            compression: None,
            connect_timeout: Some(Duration::from_secs(15)),
            timeout: Some(Duration::from_secs(15)),
            max_decoding_message_size: None,
            tls_config: None,
            tcp_nodelay: None,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlockFilters {
    pub filters: HashMap<String, SubscribeRequestFilterBlocks>,
    pub failed_transactions: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SendStatus {
    Sent,
    DroppedFull,
    DroppedClosed,
    Skipped,
}

struct TxDispatchOutcome {
    status: SendStatus,
    decode_ns: Option<u128>,
}

struct StreamLogState {
    last_log: Instant,
    last_msg: Instant,
    msg_count: u64,
    account_msgs: u64,
    transaction_msgs: u64,
    block_msgs: u64,
    ping_msgs: u64,
    updates_sent: u64,
    updates_dropped_full: u64,
    updates_dropped_closed: u64,
    updates_skipped: u64,
    tx_decode_count: u64,
    tx_decode_total_ns: u128,
    tx_decode_max_ns: u128,
    last_slot: Option<u64>,
}

impl StreamLogState {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_log: now,
            last_msg: now,
            msg_count: 0,
            account_msgs: 0,
            transaction_msgs: 0,
            block_msgs: 0,
            ping_msgs: 0,
            updates_sent: 0,
            updates_dropped_full: 0,
            updates_dropped_closed: 0,
            updates_skipped: 0,
            tx_decode_count: 0,
            tx_decode_total_ns: 0,
            tx_decode_max_ns: 0,
            last_slot: None,
        }
    }

    fn reset_window(&mut self) {
        let now = Instant::now();
        self.last_log = now;
        self.last_msg = now;
        self.msg_count = 0;
        self.account_msgs = 0;
        self.transaction_msgs = 0;
        self.block_msgs = 0;
        self.ping_msgs = 0;
        self.updates_sent = 0;
        self.updates_dropped_full = 0;
        self.updates_dropped_closed = 0;
        self.updates_skipped = 0;
        self.tx_decode_count = 0;
        self.tx_decode_total_ns = 0;
        self.tx_decode_max_ns = 0;
    }

    fn record_send_status(&mut self, status: SendStatus) {
        match status {
            SendStatus::Sent => self.updates_sent += 1,
            SendStatus::DroppedFull => self.updates_dropped_full += 1,
            SendStatus::DroppedClosed => self.updates_dropped_closed += 1,
            SendStatus::Skipped => self.updates_skipped += 1,
        }
    }

    fn record_decode_ns(&mut self, decode_ns: u128) {
        self.tx_decode_count += 1;
        self.tx_decode_total_ns += decode_ns;
        if decode_ns > self.tx_decode_max_ns {
            self.tx_decode_max_ns = decode_ns;
        }
    }

    fn maybe_log(&mut self, datasource_id: &DatasourceId, sender_capacity: usize, log_interval: Duration) {
        let elapsed = self.last_log.elapsed();
        if elapsed < log_interval {
            return;
        }

        let secs = elapsed.as_secs_f64();
        let msg_rate = if secs > 0.0 { self.msg_count as f64 / secs } else { 0.0 };
        let avg_decode_ms = if self.tx_decode_count > 0 {
            (self.tx_decode_total_ns as f64 / self.tx_decode_count as f64) / 1_000_000.0
        } else {
            0.0
        };
        let max_decode_ms = (self.tx_decode_max_ns as f64) / 1_000_000.0;
        let last_slot = self.last_slot.unwrap_or(0);
        let since_last_msg = self.last_msg.elapsed();

        log::info!(
            "yellowstone grpc stats [{}]: msgs={} (acct={}, tx={}, block={}, ping={}) updates_sent={} dropped_full={} dropped_closed={} skipped={} msg_rate={:.1}/s tx_decode_avg_ms={:.3} tx_decode_max_ms={:.3} last_slot={} since_last_msg={:?} sender_capacity={}",
            datasource_id.as_str(),
            self.msg_count,
            self.account_msgs,
            self.transaction_msgs,
            self.block_msgs,
            self.ping_msgs,
            self.updates_sent,
            self.updates_dropped_full,
            self.updates_dropped_closed,
            self.updates_skipped,
            msg_rate,
            avg_decode_ms,
            max_decode_ms,
            last_slot,
            since_last_msg,
            sender_capacity,
        );

        self.reset_window();
    }
}

impl YellowstoneGrpcGeyserClient {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        endpoint: String,
        x_token: Option<String>,
        commitment: Option<CommitmentLevel>,
        account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
        transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
        block_filters: BlockFilters,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        geyser_config: YellowstoneGrpcClientConfig,
    ) -> Self {
        YellowstoneGrpcGeyserClient {
            endpoint,
            x_token,
            commitment,
            account_filters,
            transaction_filters,
            block_filters,
            account_deletions_tracked,
            geyser_config,
        }
    }
}

impl YellowstoneGrpcClientConfig {
    pub const fn new(
        compression: Option<CompressionEncoding>,
        connect_timeout: Option<Duration>,
        timeout: Option<Duration>,
        max_decoding_message_size: Option<usize>,
        tls_config: Option<ClientTlsConfig>,
        tcp_nodelay: Option<bool>,
    ) -> Self {
        YellowstoneGrpcClientConfig {
            compression,
            connect_timeout,
            timeout,
            max_decoding_message_size,
            tls_config,
            tcp_nodelay,
        }
    }

    pub fn geyser_config_builder(
        &self,
        mut builder: GeyserGrpcBuilder,
    ) -> GeyserGrpcBuilderResult<GeyserGrpcBuilder> {
        builder = builder.connect_timeout(self.connect_timeout.unwrap_or(Duration::from_secs(15)));

        builder = builder.timeout(self.timeout.unwrap_or(Duration::from_secs(15)));
        let tls = self
            .tls_config
            .clone()
            .unwrap_or_else(|| ClientTlsConfig::new().with_enabled_roots());
        builder = builder.tls_config(tls)?;

        if let Some(compression) = self.compression {
            builder = builder
                .send_compressed(compression)
                .accept_compressed(compression);
        }
        if let Some(val) = self.max_decoding_message_size {
            builder = builder.max_decoding_message_size(val);
        }

        if let Some(val) = self.tcp_nodelay {
            builder = builder.tcp_nodelay(val);
        }
        Ok(builder)
    }
}

#[async_trait]
impl Datasource for YellowstoneGrpcGeyserClient {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let endpoint = self.endpoint.clone();
        let x_token = self.x_token.clone();
        let commitment = self.commitment;
        let account_filters = self.account_filters.clone();
        let transaction_filters = self.transaction_filters.clone();
        let account_deletions_tracked = self.account_deletions_tracked.clone();
        let BlockFilters {
            filters,
            failed_transactions: block_failed_transactions,
        } = self.block_filters.clone();
        let retain_block_failed_transactions = block_failed_transactions.unwrap_or(true);

        let builder = GeyserGrpcClient::build_from_shared(endpoint)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .x_token(x_token)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        let mut geyser_client = self
            .geyser_config
            .geyser_config_builder(builder)
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?
            .connect()
            .await
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        tokio::spawn(async move {
            let log_interval = Duration::from_secs(10);
            let stall_warn = Duration::from_secs(5);
            let mut stream_stats = StreamLogState::new();
            let subscribe_request = SubscribeRequest {
                slots: HashMap::new(),
                accounts: account_filters,
                transactions: transaction_filters,
                transactions_status: HashMap::new(),
                entry: HashMap::new(),
                blocks: filters,
                blocks_meta: HashMap::new(),
                commitment: commitment.map(|x| x as i32),
                accounts_data_slice: vec![],
                ping: None,
                from_slot: None,
            };

            let id_for_loop = id.clone();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling Yellowstone gRPC subscription.");
                        break;
                    }
                    result = geyser_client.subscribe_with_request(Some(subscribe_request.clone())) => {
                        match result {
                            Ok((mut subscribe_tx, mut stream)) => {
                                stream_stats.reset_window();
                                while let Some(message) = stream.next().await {
                                    if cancellation_token.is_cancelled() {
                                        break;
                                    }

                                    let now = Instant::now();
                                    let gap = now.duration_since(stream_stats.last_msg);
                                    if gap > stall_warn {
                                        log::warn!(
                                            "yellowstone grpc stalled [{}]: no messages for {:?} (last_slot={})",
                                            id_for_loop.as_str(),
                                            gap,
                                            stream_stats.last_slot.unwrap_or(0)
                                        );
                                    }
                                    stream_stats.last_msg = now;
                                    stream_stats.msg_count += 1;

                                    match message {
                                        Ok(msg) => match msg.update_oneof {
                                            Some(UpdateOneof::Account(account_update)) => {
                                                stream_stats.account_msgs += 1;
                                                stream_stats.last_slot = Some(account_update.slot);
                                                record_ingest_slot(&metrics, &id_for_loop, account_update.slot).await;
                                                let status = send_subscribe_account_update_info(
                                                    account_update.account,
                                                    &metrics,
                                                    &sender,
                                                    id_for_loop.clone(),
                                                    account_update.slot,
                                                    &account_deletions_tracked,
                                                )
                                                .await;
                                                stream_stats.record_send_status(status);
                                            }

                                            Some(UpdateOneof::Transaction(transaction_update)) => {
                                                stream_stats.transaction_msgs += 1;
                                                stream_stats.last_slot = Some(transaction_update.slot);
                                                record_ingest_slot(&metrics, &id_for_loop, transaction_update.slot).await;
                                                let outcome = send_subscribe_update_transaction_info(
                                                    transaction_update.transaction,
                                                    &metrics,
                                                    &sender,
                                                    id_for_loop.clone(),
                                                    transaction_update.slot,
                                                    None,
                                                )
                                                .await;
                                                stream_stats.record_send_status(outcome.status);
                                                if let Some(decode_ns) = outcome.decode_ns {
                                                    stream_stats.record_decode_ns(decode_ns);
                                                }
                                            }
                                            Some(UpdateOneof::Block(block_update)) => {
                                                stream_stats.block_msgs += 1;
                                                stream_stats.last_slot = Some(block_update.slot);
                                                record_ingest_slot(&metrics, &id_for_loop, block_update.slot).await;
                                                let block_time = block_update.block_time.map(|ts| ts.timestamp);

                                                for transaction_update in block_update.transactions {
                                                    if retain_block_failed_transactions || transaction_update.meta.as_ref().map(|meta| meta.err.is_none()).unwrap_or(false) {
                                                        let outcome = send_subscribe_update_transaction_info(
                                                            Some(transaction_update),
                                                            &metrics,
                                                            &sender,
                                                            id_for_loop.clone(),
                                                            block_update.slot,
                                                            block_time,
                                                        )
                                                        .await;
                                                        stream_stats.record_send_status(outcome.status);
                                                        if let Some(decode_ns) = outcome.decode_ns {
                                                            stream_stats.record_decode_ns(decode_ns);
                                                        }
                                                    }
                                                }

                                                for account_info in block_update.accounts {
                                                    let status = send_subscribe_account_update_info(
                                                        Some(account_info),
                                                        &metrics,
                                                        &sender,
                                                        id_for_loop.clone(),
                                                        block_update.slot,
                                                        &account_deletions_tracked,
                                                    )
                                                    .await;
                                                    stream_stats.record_send_status(status);
                                                }
                                            }

                                            Some(UpdateOneof::Ping(_)) => {
                                                stream_stats.ping_msgs += 1;
                                                match subscribe_tx
                                                    .send(SubscribeRequest {
                                                        ping: Some(SubscribeRequestPing { id: 1 }),
                                                        ..Default::default()
                                                    })
                                                    .await {
                                                        Ok(()) => (),
                                                        Err(error) => {
                                                            log::error!("Failed to send ping error: {error:?}");
                                                            break;
                                                        },
                                                    }
                                            }

                                            _ => {}
                                        },
                                        Err(error) => {
                                            log::error!("Geyser stream error: {error:?}");
                                            break;
                                        }
                                    }

                                    stream_stats.maybe_log(&id_for_loop, sender.capacity(), log_interval);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to subscribe: {e:?}");
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![
            UpdateType::AccountUpdate,
            UpdateType::Transaction,
            UpdateType::AccountDeletion,
        ]
    }
}

async fn send_subscribe_account_update_info(
    account_update_info: Option<SubscribeUpdateAccountInfo>,
    metrics: &MetricsCollection,
    sender: &Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    slot: u64,
    account_deletions_tracked: &RwLock<HashSet<Pubkey>>,
) -> SendStatus {
    let start_time = std::time::Instant::now();

    if let Some(account_info) = account_update_info {
        let Ok(account_pubkey) = Pubkey::try_from(account_info.pubkey) else {
            return SendStatus::Skipped;
        };

        let Ok(account_owner_pubkey) = Pubkey::try_from(account_info.owner) else {
            return SendStatus::Skipped;
        };

        let account = Account {
            lamports: account_info.lamports,
            data: account_info.data,
            owner: account_owner_pubkey,
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
        };

        let send_status = if account.lamports == 0
            && account.data.is_empty()
            && account_owner_pubkey == solana_system_interface::program::ID
        {
            let accounts = account_deletions_tracked.read().await;
            if accounts.contains(&account_pubkey) {
                let account_deletion = AccountDeletion {
                    pubkey: account_pubkey,
                    slot,
                    transaction_signature: account_info
                        .txn_signature
                        .and_then(|sig| Signature::try_from(sig).ok()),
                };
                match sender.try_send((Update::AccountDeletion(account_deletion), id)) {
                    Ok(()) => SendStatus::Sent,
                    Err(TrySendError::Full(_)) => {
                        log::error!(
                            "Failed to send account deletion update for pubkey {account_pubkey:?} at slot {slot}: full (capacity={})",
                            sender.capacity()
                        );
                        let _ = metrics.increment_counter("yellowstone_grpc_try_send_full", 1).await;
                        SendStatus::DroppedFull
                    }
                    Err(TrySendError::Closed(_)) => {
                        log::error!(
                            "Failed to send account deletion update for pubkey {account_pubkey:?} at slot {slot}: closed"
                        );
                        let _ = metrics.increment_counter("yellowstone_grpc_try_send_closed", 1).await;
                        SendStatus::DroppedClosed
                    }
                }
            } else {
                return SendStatus::Skipped;
            }
        } else {
            let update = Update::Account(AccountUpdate {
                pubkey: account_pubkey,
                account,
                slot,
                transaction_signature: account_info
                    .txn_signature
                    .and_then(|sig| Signature::try_from(sig).ok()),
            });

            match sender.try_send((update, id)) {
                Ok(()) => SendStatus::Sent,
                Err(TrySendError::Full(_)) => {
                    log::error!(
                        "Failed to send account update for pubkey {account_pubkey:?} at slot {slot}: full (capacity={})",
                        sender.capacity()
                    );
                    let _ = metrics.increment_counter("yellowstone_grpc_try_send_full", 1).await;
                    SendStatus::DroppedFull
                }
                Err(TrySendError::Closed(_)) => {
                    log::error!(
                        "Failed to send account update for pubkey {account_pubkey:?} at slot {slot}: closed"
                    );
                    let _ = metrics.increment_counter("yellowstone_grpc_try_send_closed", 1).await;
                    SendStatus::DroppedClosed
                }
            }
        };

        metrics
            .record_histogram(
                "yellowstone_grpc_account_process_time_nanoseconds",
                start_time.elapsed().as_nanos() as f64,
            )
            .await
            .expect("Failed to record histogram");

        metrics
            .increment_counter("yellowstone_grpc_account_updates_received", 1)
            .await
            .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

        return send_status;
    } else {
        log::error!("No account info in UpdateOneof::Account at slot {slot}");
        return SendStatus::Skipped;
    }
}

async fn record_ingest_slot(metrics: &MetricsCollection, id: &DatasourceId, slot: u64) {
    let name = format!("datasource_ingest_last_slot_{}", id.as_str());
    metrics
        .update_gauge(&name, slot as f64)
        .await
        .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));
}

async fn send_subscribe_update_transaction_info(
    transaction_info: Option<SubscribeUpdateTransactionInfo>,
    metrics: &MetricsCollection,
    sender: &Sender<(Update, DatasourceId)>,
    id: DatasourceId,
    slot: u64,
    block_time: Option<i64>,
) -> TxDispatchOutcome {
    let start_time = std::time::Instant::now();
    let decode_start = Instant::now();

    if let Some(transaction_info) = transaction_info {
        let Ok(signature) = Signature::try_from(transaction_info.signature) else {
            return TxDispatchOutcome {
                status: SendStatus::Skipped,
                decode_ns: Some(decode_start.elapsed().as_nanos()),
            };
        };
        let Some(yellowstone_transaction) = transaction_info.transaction else {
            return TxDispatchOutcome {
                status: SendStatus::Skipped,
                decode_ns: Some(decode_start.elapsed().as_nanos()),
            };
        };
        let Some(yellowstone_tx_meta) = transaction_info.meta else {
            return TxDispatchOutcome {
                status: SendStatus::Skipped,
                decode_ns: Some(decode_start.elapsed().as_nanos()),
            };
        };
        let Ok(versioned_transaction) = create_tx_versioned(yellowstone_transaction) else {
            return TxDispatchOutcome {
                status: SendStatus::Skipped,
                decode_ns: Some(decode_start.elapsed().as_nanos()),
            };
        };
        let meta_original = match create_tx_meta(yellowstone_tx_meta) {
            Ok(meta) => meta,
            Err(err) => {
                log::error!("Failed to create transaction meta: {err:?}");
                return TxDispatchOutcome {
                    status: SendStatus::Skipped,
                    decode_ns: Some(decode_start.elapsed().as_nanos()),
                };
            }
        };
        let decode_ns = decode_start.elapsed().as_nanos();
        let update = Update::Transaction(Box::new(TransactionUpdate {
            signature,
            transaction: versioned_transaction,
            meta: meta_original,
            is_vote: transaction_info.is_vote,
            slot,
            index: Some(transaction_info.index),
            block_time,
            block_hash: None,
        }));
        let send_status = match sender.try_send((update, id)) {
            Ok(()) => SendStatus::Sent,
            Err(TrySendError::Full(_)) => {
                log::error!(
                    "Failed to send transaction update with signature {signature:?} at slot {slot}: full (capacity={})",
                    sender.capacity()
                );
                let _ = metrics.increment_counter("yellowstone_grpc_try_send_full", 1).await;
                SendStatus::DroppedFull
            }
            Err(TrySendError::Closed(_)) => {
                log::error!(
                    "Failed to send transaction update with signature {signature:?} at slot {slot}: closed"
                );
                let _ = metrics.increment_counter("yellowstone_grpc_try_send_closed", 1).await;
                SendStatus::DroppedClosed
            }
        };

        if send_status != SendStatus::Sent {
            return TxDispatchOutcome {
                status: send_status,
                decode_ns: Some(decode_ns),
            };
        }

        metrics
            .record_histogram(
                "yellowstone_grpc_transaction_process_time_nanoseconds",
                start_time.elapsed().as_nanos() as f64,
            )
            .await
            .expect("Failed to record histogram");

        metrics
            .increment_counter("yellowstone_grpc_transaction_updates_received", 1)
            .await
            .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

        return TxDispatchOutcome {
            status: send_status,
            decode_ns: Some(decode_ns),
        };
    }

    log::error!("No transaction info in `UpdateOneof::Transaction` at slot {slot}");
    TxDispatchOutcome {
        status: SendStatus::Skipped,
        decode_ns: Some(decode_start.elapsed().as_nanos()),
    }
}
