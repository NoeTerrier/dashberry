use dioxus::prelude::*;
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
pub async fn system_info() -> anyhow::Result<System> {
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
