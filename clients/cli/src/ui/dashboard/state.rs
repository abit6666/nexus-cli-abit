//! Dashboard state management
//!
//! Contains the main dashboard state struct and related enums

use crate::consts::cli_consts::MAX_ACTIVITY_LOGS;
use crate::environment::Environment;
use crate::events::{Event as WorkerEvent, ProverState};
use crate::ui::app::UIConfig;
use crate::ui::metrics::{SystemMetrics, TaskFetchInfo, ZkVMMetrics};

use std::collections::VecDeque;
use std::time::Instant;
use sysinfo::System;

/// State for tracking fetching operations
#[derive(Debug, Clone)]
pub enum FetchingState {
    Idle,
    Active { started_at: Instant },
    Timeout,
}

/// Enhanced dashboard state with real-time metrics and animations.
#[derive(Debug)]
pub struct DashboardState {
    pub node_id: Option<u64>,
    pub environment: Environment,
    pub start_time: Instant,
    pub last_task: Option<String>,
    pub current_task: Option<String>,
    pub total_ram_gb: f64,
    pub num_threads: usize,
    pub pending_events: VecDeque<WorkerEvent>,
    pub activity_logs: VecDeque<WorkerEvent>,
    pub system_metrics: SystemMetrics,
    pub zkvm_metrics: ZkVMMetrics,
    pub task_fetch_info: TaskFetchInfo,
    pub tick: usize,
    pub cpu_history: Vec<u64>, // Field for CPU chart data
    pub ram_history: Vec<u64>, // Field for RAM chart data
    fetching_state: FetchingState,
    sysinfo: System,
    current_prover_state: ProverState,
    pub step2_start_time: Option<Instant>,
    pub waiting_start_info: Option<(Instant, u64)>,
}

impl DashboardState {
    /// Creates a new instance of the dashboard state.
    pub fn new(
        node_id: Option<u64>,
        environment: Environment,
        start_time: Instant,
        ui_config: UIConfig,
    ) -> Self {
        let mut system_metrics = SystemMetrics::default();
        system_metrics.gflops = ui_config.gflops; // Set initial GFLOPs

        Self {
            node_id,
            environment,
            start_time,
            last_task: None,
            current_task: None,
            total_ram_gb: crate::system::total_memory_gb(),
            num_threads: ui_config.num_threads,
            pending_events: VecDeque::new(),
            activity_logs: VecDeque::new(),
            system_metrics,
            zkvm_metrics: ZkVMMetrics::default(),
            task_fetch_info: TaskFetchInfo::default(),
            tick: 0,
            cpu_history: vec![0; 60], // Initialize with 60 zero-values
            ram_history: vec![0; 60], // Initialize with 60 zero-values
            fetching_state: FetchingState::Idle,
            sysinfo: System::new_all(),
            current_prover_state: ProverState::Waiting,
            step2_start_time: None,
            waiting_start_info: None,
        }
    }

    pub fn fetching_state(&self) -> &FetchingState {
        &self.fetching_state
    }

    pub fn set_fetching_state(&mut self, state: FetchingState) {
        self.fetching_state = state;
    }

    pub fn current_prover_state(&self) -> ProverState {
        self.current_prover_state
    }

    pub fn set_current_prover_state(&mut self, state: ProverState) {
        self.current_prover_state = state;
    }

    pub fn get_sysinfo_mut(&mut self) -> &mut System {
        &mut self.sysinfo
    }

    pub fn add_to_activity_log(&mut self, event: WorkerEvent) {
        if self.activity_logs.len() >= MAX_ACTIVITY_LOGS {
            self.activity_logs.pop_front();
        }
        self.activity_logs.push_back(event);
    }

    pub fn add_event(&mut self, event: WorkerEvent) {
        self.pending_events.push_back(event);
    }
}