use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

use dioxus::prelude::*;
use env_file_reader::read_file;
use serde::Deserialize;
use serde::Serialize;

/* -------------------------------------------------------------------------- */
/*                                   System                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct System {
    pub hostname: String,
    pub product_name: String,
    pub os: String,
    pub kernel: String,
    pub uptime: u64,
    pub cpu_cores: usize,
}

#[get("/api/system_info")]
pub async fn system_info() -> Result<System> {
    use crate::dioxus_core::anyhow;
    Ok(System {
        hostname: sysinfo::System::host_name().ok_or(anyhow!("No hostname"))?,
        product_name: sysinfo::Product::name().ok_or(anyhow!("No product name"))?,
        os: sysinfo::System::long_os_version().ok_or(anyhow!("No OS version"))?,
        kernel: sysinfo::System::kernel_long_version(),
        uptime: sysinfo::System::uptime(),
        cpu_cores: sysinfo::System::physical_core_count().ok_or(anyhow!("No cpu cores"))?,
    })
}

/* -------------------------------------------------------------------------- */
/*                                 Temperature                                */
/* -------------------------------------------------------------------------- */

#[get("/api/temperature_info")]
pub async fn temperature_info() -> Result<f32> {
    let components = sysinfo::Components::new_with_refreshed_list();

    let temperature = components
        .iter()
        .map(|c| c.temperature().unwrap_or_default())
        .sum::<f32>()
        / (components.len() as f32);

    Ok(temperature)
}

/* -------------------------------------------------------------------------- */
/*                                   Memory                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Memory {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

#[get("/api/memory_info")]
pub async fn memory_info() -> Result<Memory> {
    let mut system = sysinfo::System::new();
    system.refresh_memory_specifics(sysinfo::MemoryRefreshKind::nothing().with_ram().with_swap());

    Ok(Memory {
        total_memory: system.total_memory(),
        used_memory: system.used_memory(),
        total_swap: system.total_swap(),
        used_swap: system.used_swap(),
    })
}

/* -------------------------------------------------------------------------- */
/*                                    Disks                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Disk {
    pub kind: String,
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub available_space: u64,
    pub total_space: u64,
}

pub type Disks = Vec<Disk>;

#[get("/api/disk_info")]
pub async fn disks_info() -> Result<Disks> {
    let mut disks = sysinfo::Disks::new();

    disks.refresh(false);

    Ok(disks
        .list()
        .iter()
        .map(|disk| Disk {
            kind: disk.kind().to_string(),
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().display().to_string(),
            file_system: disk.file_system().display().to_string(),
            available_space: disk.available_space(),
            total_space: disk.total_space(),
        })
        .collect::<Disks>())
}

/* -------------------------------------------------------------------------- */
/*                                   Docker                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Container {
    pub name: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
    pub project_name: Option<String>,
}

pub type Containers = Vec<Container>;

#[get("/api/docker_info")]
pub async fn docker_info() -> Result<Containers> {
    let containers = dockers::Container::list()?;

    Ok(containers
        .into_iter()
        .map(|c| Container {
            name: c.Names,
            image: c.Image,
            state: c.State,
            status: c.Status,
            project_name: c.Labels.and_then(|h| {
                h.get("com.docker.compose.project")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            }),
        })
        .collect())
}

/* -------------------------------------------------------------------------- */
/*                                   Network                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ipv4 {
    pub addr: String,
    pub is_loopback: bool,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ipv6 {
    pub addr: String,
    pub is_loopback: bool,
    pub is_unicast_link_local: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interface {
    pub name: String,
    pub mac_addr: Option<String>,
    pub ipv4: Vec<Ipv4>,
    pub ipv6: Vec<Ipv6>,
    pub is_up: bool,
}

#[get("/api/network_info")]
pub async fn network_info() -> Result<Vec<Interface>> {
    let interfaces = netdev::interface::get_interfaces()
        .into_iter()
        .map(|interface| Interface {
            name: interface.name.clone(),
            mac_addr: interface.mac_addr.map(|a| a.address()),
            is_up: interface.is_up(),
            ipv4: interface
                .ipv4
                .iter()
                .map(|a| Ipv4 {
                    addr: a.to_string(),
                    is_loopback: a.addr().is_loopback(),
                    is_private: a.addr().is_private(),
                })
                .collect(),
            ipv6: interface
                .ipv6
                .iter()
                .map(|a| Ipv6 {
                    addr: a.to_string(),
                    is_loopback: a.addr().is_loopback(),
                    is_unicast_link_local: a.addr().is_unicast_link_local(),
                })
                .collect(),
        })
        .filter(|i| !i.ipv4.is_empty() || !i.ipv6.is_empty())
        .collect();

    Ok(interfaces)
}

/* -------------------------------------------------------------------------- */
/*                                    NGINX                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxLogLine {
    pub timestamp: String,
    pub pid: String,
    pub client_ip: String,
    pub request_id: String,
    pub http_method: String,
    pub http_path: String,
    pub protocol: String,
    pub host: String,
    pub user_agent: String,
    pub referer: String,
    pub status_code: u16,
    pub bytes_sent: u32,
    pub request_time_secs: f32,
}

impl Display for NginxLogLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(f.write_fmt(format_args!("{self:?}"))?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxMetrics {
    pub logs: Vec<NginxLogLine>,
    pub status_codes: Vec<(u16, u32)>,
    pub http_paths: Vec<(String, u32)>,
    pub hosts: Vec<(String, u32)>,
    pub user_agents: Vec<(String, u32)>,
}

const MAX_NGINX_LOGS: usize = 100;

#[get("/api/nginx")]
pub async fn nginx_status() -> Result<NginxMetrics> {
    let env_variables = read_file(".env")?;

    let nginx_log_path = env_variables["NGINX_LOG_PATH"].clone();

    let logs = fs::read_to_string(nginx_log_path)?;
    let last_logs = logs
        .lines()
        .take(MAX_NGINX_LOGS)
        .map(|line| serde_json::from_str::<NginxLogLine>(line).unwrap())
        .collect();

    let logs = logs
        .lines()
        .map(|line| serde_json::from_str::<NginxLogLine>(line).unwrap());

    let mut status_map = HashMap::<u16, u32>::new();
    let mut http_path_map = HashMap::<String, u32>::new();
    let mut host_map = HashMap::<String, u32>::new();
    let mut user_agent_map = HashMap::<String, u32>::new();

    for log in logs {
        *http_path_map.entry(log.http_path).or_insert_with(|| 0) += 1;
        *status_map.entry(log.status_code).or_insert_with(|| 0) += 1;
        *host_map.entry(log.host).or_insert_with(|| 0) += 1;
        *user_agent_map.entry(log.user_agent).or_insert_with(|| 0) += 1;
    }

    let mut http_paths = http_path_map.into_iter().collect::<Vec<_>>();
    http_paths.sort_by_key(|p| p.1);
    http_paths.reverse();

    let mut status_codes = status_map.into_iter().collect::<Vec<_>>();
    status_codes.sort_by_key(|p| p.1);
    status_codes.reverse();

    let mut user_agents = user_agent_map.into_iter().collect::<Vec<_>>();
    user_agents.sort_by_key(|p| p.1);
    user_agents.reverse();

    let mut hosts = host_map.into_iter().collect::<Vec<_>>();
    hosts.sort_by_key(|p| p.1);
    hosts.reverse();

    let test = NginxMetrics {
        logs: last_logs,
        status_codes,
        http_paths,
        hosts,
        user_agents,
    };

    Ok(test)
}
