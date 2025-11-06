use crate::variants::{
    _macros::domain_m_o_r, DomainDependency, DomainMap, PowerDomain, RockchipDomainInfo,
    RockchipPmuInfo,
};

// QoS (Quality of Service) base addresses for RK3588
// These addresses are used for bandwidth and priority control
const QOS_GPU_BASE: usize = 0xFDF35000;
const QOS_NPU_BASE: usize = 0xFDF40000;
const QOS_RKVDEC_BASE: usize = 0xFDF48000;
const QOS_RKVENC_BASE: usize = 0xFDF50000;
const QOS_VOP_BASE: usize = 0xFDF60000;
const QOS_VI_BASE: usize = 0xFDF70000;
const QOS_VCODEC_BASE: usize = 0xFDF78000;

// QoS port offset arrays for each domain
// Each port has its own set of QoS registers (PRIORITY, MODE, BANDWIDTH, SATURATION, EXTCONTROL)
static GPU_QOS_OFFSETS: &[usize] = &[QOS_GPU_BASE, QOS_GPU_BASE + 0x1000];

static NPU_QOS_OFFSETS: &[usize] = &[
    QOS_NPU_BASE,
    QOS_NPU_BASE + 0x1000,
    QOS_NPU_BASE + 0x2000,
    QOS_NPU_BASE + 0x3000,
];

static RKVDEC_QOS_OFFSETS: &[usize] = &[QOS_RKVDEC_BASE, QOS_RKVDEC_BASE + 0x1000];

static RKVENC_QOS_OFFSETS: &[usize] = &[QOS_RKVENC_BASE, QOS_RKVENC_BASE + 0x1000];

static VOP_QOS_OFFSETS: &[usize] = &[
    QOS_VOP_BASE,
    QOS_VOP_BASE + 0x1000,
    QOS_VOP_BASE + 0x2000,
    QOS_VOP_BASE + 0x3000,
];

static VI_QOS_OFFSETS: &[usize] = &[QOS_VI_BASE, QOS_VI_BASE + 0x1000];

static VCODEC_QOS_OFFSETS: &[usize] = &[
    QOS_VCODEC_BASE,
    QOS_VCODEC_BASE + 0x1000,
    QOS_VCODEC_BASE + 0x2000,
];

define_power_domains! {
    // VD_NPU
    /// NPU (Neural Processing Unit) main domain
    NPU = 8,
    /// NPU top-level domain
    NPUTOP = 9,
    /// NPU core 1 domain
    NPU1 = 10,
    /// NPU core 2 domain
    NPU2 = 11,

    // VD_GPU
    /// GPU (Graphics Processing Unit) power domain
    GPU = 12,

    // VD_VCODEC
    /// Video codec domain
    VCODEC = 13,
    /// Rockchip video decoder 0
    RKVDEC0 = 14,
    /// Rockchip video decoder 1
    RKVDEC1 = 15,
    /// Video encoder 0
    VENC0 = 16,
    /// Video encoder 1
    VENC1 = 17,

    // VD_LOGIC
    /// Video processing unit
    VDPU = 21,
    /// Raster Graphic Acceleration 3.0
    RGA30 = 22,
    /// AV1 video codec
    AV1 = 23,
    /// Video Output Processor
    VOP = 24,
    /// Video Output 0
    VO0 = 25,
    /// Video Output 1
    VO1 = 26,
    /// Video Input
    VI = 27,
    /// Image Signal Processor 1
    ISP1 = 28,
    /// Forward Error Correction
    FEC = 29,
    /// Raster Graphic Acceleration 3.1
    RGA31 = 30,
    /// USB controller
    USB = 31,
    /// PCIe, SATA, USB3 combo PHY
    PHP = 32,
    /// Gigabit Ethernet MAC
    GMAC = 33,
    /// PCIe controller
    PCIE = 34,
    /// Non-Volatile Memory
    NVM = 35,
    /// Non-Volatile Memory 0
    NVM0 = 36,
    /// SDIO controller
    SDIO = 37,
    /// Audio subsystem
    AUDIO = 38,
    /// SD/MMC controller
    SDMMC = 40,
}

pub fn pmu_info() -> RockchipPmuInfo {
    RockchipPmuInfo {
        pwr_offset: 0x14c,
        status_offset: 0x180,
        req_offset: 0x10c,
        idle_offset: 0x120,
        ack_offset: 0x118,
        mem_pwr_offset: 0x1a0,
        chain_status_offset: 0x1f0,
        mem_status_offset: 0x1f8,
        repair_status_offset: 0x290,
        domains: domains(),
        ..Default::default()
    }
}

fn domain_info(
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
    wakeup: bool,
) -> RockchipDomainInfo {
    domain_m_o_r(
        name,
        pwr_offset,
        pwr,
        status,
        mem_offset,
        mem_status,
        repair_status,
        req_offset,
        req,
        idle,
        idle,
        wakeup,
        false,
    )
}

/// Create domain info with dependency configuration
fn domain_info_with_deps(
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
    wakeup: bool,
    dependency: Option<DomainDependency>,
) -> RockchipDomainInfo {
    let mut info = domain_m_o_r(
        name,
        pwr_offset,
        pwr,
        status,
        mem_offset,
        mem_status,
        repair_status,
        req_offset,
        req,
        idle,
        idle,
        wakeup,
        false,
    );
    info.dependency = dependency;
    info
}

/// Create domain info with dependency and QoS configuration
fn domain_info_with_deps_qos(
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
    wakeup: bool,
    dependency: Option<DomainDependency>,
    qos_offsets: &'static [usize],
) -> RockchipDomainInfo {
    let mut info = domain_m_o_r(
        name,
        pwr_offset,
        pwr,
        status,
        mem_offset,
        mem_status,
        repair_status,
        req_offset,
        req,
        idle,
        idle,
        wakeup,
        false,
    );
    info.dependency = dependency;
    // Convert usize offsets to u32 (safe on 32-bit and 64-bit systems for memory-mapped addresses)
    info.qos_offsets = unsafe { core::mem::transmute(qos_offsets) };
    info.num_qos = qos_offsets.len();
    info
}

/// Create domain info with QoS configuration only (no dependencies)
fn domain_info_with_qos(
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
    wakeup: bool,
    qos_offsets: &'static [usize],
) -> RockchipDomainInfo {
    let mut info = domain_m_o_r(
        name,
        pwr_offset,
        pwr,
        status,
        mem_offset,
        mem_status,
        repair_status,
        req_offset,
        req,
        idle,
        idle,
        wakeup,
        false,
    );
    info.qos_offsets = unsafe { core::mem::transmute(qos_offsets) };
    info.num_qos = qos_offsets.len();
    info
}

fn domains() -> DomainMap {
    map! {
        // GPU domain with QoS configuration (2 ports)
        GPU      => domain_info_with_qos("gpu", 0x0, bit!(0), 0, 0x0, 0, bit!(1), 0x0, bit!(0), bit!(0), false, GPU_QOS_OFFSETS),

        // NPU domains with dependencies and QoS
        NPU      => domain_info_with_qos("npu", 0x0, bit!(1), bit!(1), 0x0, 0, 0, 0x0, 0, 0, false, NPU_QOS_OFFSETS),

        // VCODEC domain with QoS and dependencies (parent of VENC0/1, RKVDEC0/1)
        VCODEC   => domain_info_with_deps_qos("vcodec", 0x0, bit!(2), bit!(2), 0x0, 0, 0, 0x0, 0, 0, false,
                        Some(DomainDependency {
                            parent: None,
                            children: alloc::vec![VENC0, VENC1, RKVDEC0, RKVDEC1],
                        }), VCODEC_QOS_OFFSETS),

        // NPUTOP has NPU1 and NPU2 as children (children must be powered off first)
        NPUTOP   => domain_info_with_deps("nputop", 0x0, bit!(3), 0, 0x0, bit!(11), bit!(2), 0x0, bit!(1), bit!(1), false,
                        Some(DomainDependency {
                            parent: None,
                            children: alloc::vec![NPU1, NPU2],
                        })),

        // NPU1 depends on NPUTOP (parent must be powered on first)
        NPU1     => domain_info_with_deps("npu1", 0x0, bit!(4), 0, 0x0, bit!(12), bit!(3), 0x0, bit!(2), bit!(2), false,
                        Some(DomainDependency {
                            parent: Some(NPUTOP),
                            children: alloc::vec![],
                        })),

        // NPU2 depends on NPUTOP (parent must be powered on first)
        NPU2     => domain_info_with_deps("npu2", 0x0, bit!(5), 0, 0x0, bit!(13), bit!(4), 0x0, bit!(3), bit!(3), false,
                        Some(DomainDependency {
                            parent: Some(NPUTOP),
                            children: alloc::vec![],
                        })),

        // Video encoder domains with dependencies (children of VCODEC)
        VENC0    => domain_info_with_deps_qos("venc0", 0x0, bit!(6), 0, 0x0, bit!(14), bit!(5), 0x0, bit!(4), bit!(4), false,
                        Some(DomainDependency {
                            parent: Some(VCODEC),
                            children: alloc::vec![],
                        }), RKVENC_QOS_OFFSETS),

        VENC1    => domain_info_with_deps("venc1", 0x0, bit!(7), 0, 0x0, bit!(15), bit!(6), 0x0, bit!(5), bit!(5), false,
                        Some(DomainDependency {
                            parent: Some(VCODEC),
                            children: alloc::vec![],
                        })),

        // Video decoder domains with dependencies (children of VCODEC)
        RKVDEC0  => domain_info_with_deps_qos("rkvdec0", 0x0, bit!(8), 0, 0x0, bit!(16), bit!(7), 0x0, bit!(6), bit!(6), false,
                        Some(DomainDependency {
                            parent: Some(VCODEC),
                            children: alloc::vec![],
                        }), RKVDEC_QOS_OFFSETS),

        RKVDEC1  => domain_info_with_deps("rkvdec1", 0x0, bit!(9), 0, 0x0, bit!(17), bit!(8), 0x0, bit!(7), bit!(7), false,
                        Some(DomainDependency {
                            parent: Some(VCODEC),
                            children: alloc::vec![],
                        })),

        // LOGIC domains
        VDPU     => domain_info("vdpu",    0x0, bit!(10), 0,       0x0, bit!(18), bit!(9),  0x0, bit!(8),  bit!(8),  false),
        RGA30    => domain_info("rga30",   0x0, bit!(11), 0,       0x0, bit!(19), bit!(10), 0x0, 0,        0,        false),
        AV1      => domain_info("av1",     0x0, bit!(12), 0,       0x0, bit!(20), bit!(11), 0x0, bit!(9),  bit!(9),  false),

        // VI (Video Input) domain with QoS and dependencies (parent of ISP1)
        VI       => domain_info_with_deps_qos("vi", 0x0, bit!(13), 0, 0x0, bit!(21), bit!(12), 0x0, bit!(10), bit!(10), false,
                        Some(DomainDependency {
                            parent: None,
                            children: alloc::vec![ISP1],
                        }), VI_QOS_OFFSETS),

        FEC      => domain_info("fec",     0x0, bit!(14), 0,       0x0, bit!(22), bit!(13), 0x0, 0,        0,        false),

        // ISP1 depends on VI (parent must be powered on first)
        ISP1     => domain_info_with_deps("isp1", 0x0, bit!(15), 0, 0x0, bit!(23), bit!(14), 0x0, bit!(11), bit!(11), false,
                        Some(DomainDependency {
                            parent: Some(VI),
                            children: alloc::vec![],
                        })),

        // More LOGIC domains with pwr_offset 0x4
        RGA31    => domain_info("rga31",   0x4, bit!(0),  0,       0x0, bit!(24), bit!(15), 0x0, bit!(12), bit!(12), false),

        // VOP (Video Output Processor) with QoS and dependencies (parent of VO0, VO1)
        VOP      => domain_info_with_deps_qos("vop", 0x4, bit!(1), 0, 0x0, bit!(25), bit!(16), 0x0, bit!(13) | bit!(14), bit!(13) | bit!(14), false,
                        Some(DomainDependency {
                            parent: None,
                            children: alloc::vec![VO0, VO1],
                        }), VOP_QOS_OFFSETS),

        // VO0 depends on VOP (parent must be powered on first)
        VO0      => domain_info_with_deps("vo0", 0x4, bit!(2), 0, 0x0, bit!(26), bit!(17), 0x0, bit!(15), bit!(15), false,
                        Some(DomainDependency {
                            parent: Some(VOP),
                            children: alloc::vec![],
                        })),

        // VO1 depends on VOP (parent must be powered on first)
        VO1      => domain_info_with_deps("vo1", 0x4, bit!(3), 0, 0x0, bit!(27), bit!(18), 0x4, bit!(0), bit!(16), false,
                        Some(DomainDependency {
                            parent: Some(VOP),
                            children: alloc::vec![],
                        })),

        AUDIO    => domain_info("audio",   0x4, bit!(4),  0,       0x0, bit!(28), bit!(19), 0x4, bit!(1),  bit!(17), false),
        PHP      => domain_info("php",     0x4, bit!(5),  0,       0x0, bit!(29), bit!(20), 0x4, bit!(5),  bit!(21), false),
        GMAC     => domain_info("gmac",    0x4, bit!(6),  0,       0x0, bit!(30), bit!(21), 0x0, 0,        0,        false),
        PCIE     => domain_info("pcie",    0x4, bit!(7),  0,       0x0, bit!(31), bit!(22), 0x0, 0,        0,        true),
        NVM      => domain_info("nvm",     0x4, bit!(8),  bit!(24),0x4, 0,        0,        0x4, bit!(2),  bit!(18), false),
        NVM0     => domain_info("nvm0",    0x4, bit!(9),  0,       0x4, bit!(1),  bit!(23), 0x0, 0,        0,        false),
        SDIO     => domain_info("sdio",    0x4, bit!(10), 0,       0x4, bit!(2),  bit!(24), 0x4, bit!(3),  bit!(19), false),
        USB      => domain_info("usb",     0x4, bit!(11), 0,       0x4, bit!(3),  bit!(25), 0x4, bit!(4),  bit!(20), true),
        SDMMC    => domain_info("sdmmc",   0x4, bit!(13), 0,       0x4, bit!(5),  bit!(26), 0x0, 0,        0,        false),
    }
}
