#[macro_export(local_inner_macros)]
macro_rules! map {
    // 空 map
    () => {
        {
            ::alloc::collections::BTreeMap::new()
        }
    };
    // 支持多个键值对
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {{
        let mut map = ::alloc::collections::BTreeMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

/// Define power domain constants with documentation
macro_rules! define_power_domains {
    (
        $(
            $(#[$meta:meta])*
            $name:ident = $id:expr
        ),* $(,)?
    ) => {
        $(
            $(#[$meta])*
            pub const $name: PowerDomain = PowerDomain($id);
        )*
    };
}

macro_rules! bit {
    ($n:expr) => {
        (1 << $n)
    };
    () => {};
}

// Make sure RockchipDomainInfo is in scope
use super::RockchipDomainInfo;

/// DOMAIN_M macro equivalent - simpler domain definition for chips like RK3568
///
/// This corresponds to the Linux kernel's DOMAIN_M macro:
/// ```c
/// #define DOMAIN_M(_name, pwr, status, req, idle, ack, wakeup, keepon)
/// ```
#[allow(clippy::too_many_arguments)]
pub fn domain_m(
    name: &'static str,
    pwr: i32,
    status: i32,
    req: i32,
    idle: i32,
    ack: i32,
    wakeup: bool,
    keepon: bool,
) -> RockchipDomainInfo {
    RockchipDomainInfo {
        name,
        pwr_w_mask: pwr << 16,
        pwr_mask: pwr,
        status_mask: status,
        req_w_mask: req << 16,
        req_mask: req,
        idle_mask: idle,
        ack_mask: ack,
        active_wakeup: wakeup,
        keepon_startup: keepon,
        ..Default::default()
    }
}

/// DOMAIN_M_O_R macro equivalent - complex domain definition for chips like RK3588
///
/// This corresponds to the Linux kernel's DOMAIN_M_O_R macro with memory and repair support:
/// ```c
/// #define DOMAIN_M_O_R(_name, p_offset, pwr, status, m_offset, m_status, r_status, r_offset, req, idle, ack, wakeup, keepon)
/// ```
#[allow(clippy::too_many_arguments)]
pub fn domain_m_o_r(
    name: &'static str,
    pwr_offset: u32,
    pwr: i32,
    status: i32,
    mem_offset: u32,
    mem_status: i32,
    repair_status: i32,
    req_offset: u32,
    req: i32,
    idle: i32,
    ack: i32,
    wakeup: bool,
    keepon: bool,
) -> RockchipDomainInfo {
    RockchipDomainInfo {
        name,
        pwr_offset,
        pwr_w_mask: (pwr << 16),
        pwr_mask: pwr,
        status_mask: status,
        mem_offset,
        mem_status_mask: mem_status,
        repair_status_mask: repair_status,
        req_offset,
        req_w_mask: (req << 16),
        req_mask: req,
        idle_mask: idle,
        ack_mask: ack,
        active_wakeup: wakeup,
        keepon_startup: keepon,
        ..Default::default()
    }
}
