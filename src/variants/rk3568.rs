use crate::variants::{
    _macros::domain_m, DomainDependency, DomainMap, PowerDomain, RockchipDomainInfo,
    RockchipPmuInfo,
};

// QoS (Quality of Service) base addresses for RK3568
// These addresses are used for bandwidth and priority control
const QOS_GPU_BASE: usize = 0xFE128000;
const QOS_NPU_BASE: usize = 0xFE138000;
const QOS_VPU_BASE: usize = 0xFE148000;
const QOS_RKVDEC_BASE: usize = 0xFE158000;
const QOS_RKVENC_BASE: usize = 0xFE168000;

// QoS port offset arrays for each domain
// Each port has its own set of QoS registers
static GPU_QOS_OFFSETS: &[usize] = &[QOS_GPU_BASE];

static NPU_QOS_OFFSETS: &[usize] = &[QOS_NPU_BASE];

static VPU_QOS_OFFSETS: &[usize] = &[QOS_VPU_BASE, QOS_VPU_BASE + 0x1000];

static RKVDEC_QOS_OFFSETS: &[usize] = &[QOS_RKVDEC_BASE];

static RKVENC_QOS_OFFSETS: &[usize] = &[QOS_RKVENC_BASE];

define_power_domains! {
    /// NPU (Neural Processing Unit) power domain
    NPU = 6,
    /// GPU (Graphics Processing Unit) power domain
    GPU = 7,
    /// VI (Video Input) power domain
    VI = 8,
    /// VO (Video Output) power domain
    VO = 9,
    /// RGA (Raster Graphic Acceleration) power domain
    RGA = 10,
    /// VPU (Video Processing Unit) power domain
    VPU = 11,
    /// RKVDEC (Rockchip Video Decoder) power domain
    RKVDEC = 13,
    /// RKVENC (Rockchip Video Encoder) power domain
    RKVENC = 14,
    /// PIPE (Display Pipeline) power domain
    PIPE = 15,
}

/// Create domain info with QoS configuration
fn domain_m_with_qos(
    name: &'static str,
    pwr: i32,
    status: i32,
    req: i32,
    idle: i32,
    ack: i32,
    wakeup: bool,
    keepon: bool,
    qos_offsets: &'static [usize],
) -> RockchipDomainInfo {
    let mut info = domain_m(name, pwr, status, req, idle, ack, wakeup, keepon);
    info.qos_offsets = unsafe { core::mem::transmute(qos_offsets) };
    info.num_qos = qos_offsets.len();
    info
}

/// Create domain info with both QoS and dependency configuration
fn domain_m_with_deps_qos(
    name: &'static str,
    pwr: i32,
    status: i32,
    req: i32,
    idle: i32,
    ack: i32,
    wakeup: bool,
    keepon: bool,
    dependency: Option<DomainDependency>,
    qos_offsets: &'static [usize],
) -> RockchipDomainInfo {
    let mut info = domain_m(name, pwr, status, req, idle, ack, wakeup, keepon);
    info.dependency = dependency;
    info.qos_offsets = unsafe { core::mem::transmute(qos_offsets) };
    info.num_qos = qos_offsets.len();
    info
}

pub fn pmu_info() -> RockchipPmuInfo {
    RockchipPmuInfo {
        pwr_offset: 0xa0,
        status_offset: 0x98,
        req_offset: 0x50,
        idle_offset: 0x68,
        ack_offset: 0x60,
        mem_pwr_offset: 0,
        chain_status_offset: 0,
        mem_status_offset: 0,
        repair_status_offset: 0,
        domains: domains(),
        ..Default::default()
    }
}

fn domains() -> DomainMap {
    map! {
        // GPU domain with QoS (1 port)
        GPU    => domain_m_with_qos("gpu", bit!(0), bit!(0), bit!(1), bit!(1), bit!(1), false, false, GPU_QOS_OFFSETS),

        // NPU domain with QoS (1 port)
        NPU    => domain_m_with_qos("npu", bit!(1), bit!(1), bit!(2), bit!(2), bit!(2), false, false, NPU_QOS_OFFSETS),

        // VPU domain with QoS and dependencies (2 ports, parent of RKVDEC and RKVENC)
        VPU    => domain_m_with_deps_qos("vpu", bit!(2), bit!(2), bit!(6), bit!(6), bit!(6), false, false,
                    Some(DomainDependency {
                        parent: None,
                        children: alloc::vec![RKVDEC, RKVENC],
                    }), VPU_QOS_OFFSETS),

        // VI (Video Input) domain - independent
        VI     => domain_m("vi", bit!(6), bit!(6), bit!(3), bit!(3), bit!(3), false, false),

        // VO (Video Output) domain - keepon_startup=true
        VO     => domain_m("vo", bit!(7), bit!(7), bit!(4), bit!(4), bit!(4), false, true),

        // RGA (Raster Graphics) domain - independent
        RGA    => domain_m("rga", bit!(5), bit!(5), bit!(5), bit!(5), bit!(5), false, false),

        // RKVDEC (Video Decoder) with QoS and dependency (child of VPU)
        RKVDEC => domain_m_with_deps_qos("rkvdec", bit!(4), bit!(4), bit!(8), bit!(8), bit!(8), false, false,
                    Some(DomainDependency {
                        parent: Some(VPU),
                        children: alloc::vec![],
                    }), RKVDEC_QOS_OFFSETS),

        // RKVENC (Video Encoder) with QoS and dependency (child of VPU)
        RKVENC => domain_m_with_deps_qos("rkvenc", bit!(3), bit!(3), bit!(7), bit!(7), bit!(7), false, false,
                    Some(DomainDependency {
                        parent: Some(VPU),
                        children: alloc::vec![],
                    }), RKVENC_QOS_OFFSETS),

        // PIPE (Display Pipeline) - independent
        PIPE   => domain_m("pipe", bit!(8), bit!(8), bit!(11), bit!(11), bit!(11), false, false),
    }
}
