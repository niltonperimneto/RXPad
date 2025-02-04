// Conditional compilation for debug
#[cfg(debug_assertions)]
const DEBUG: bool = true;

// Linux constants module organization
mod linux {
    // Input subsystem constants
    pub mod input {
        pub use linux_input_sys::abs::*;
        pub use linux_input_sys::evdev::*;
    }
    
    // Module parameter permissions
    pub mod stat {
        pub const S_IRUGO: i32 = 0o400;  // Read permission for user/group/others
        pub const S_IWUSR: i32 = 0o200;  // Write permission for user
    }
}

// Explicit imports for clarity
use linux::input::{ABS_X, ABS_Y, ABS_Z, ABS_RZ, ABS_HAT0X, ABS_HAT0Y};
use linux::stat::{S_IRUGO, S_IWUSR};
use std::sync::atomic::{AtomicBool, Ordering};

// Network protocol constants
const XPAD_PKT_LEN: usize = 64;
const GHL_GUITAR_POKE_INTERVAL: u64 = 8; // Seconds

bitflags::bitflags! {
    /// Configuration flags for controller mapping
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MapFlags: u8 {
        const DPAD_TO_BUTTONS    = 1 << 0;
        const TRIGGERS_TO_BUTTONS = 1 << 1;
        const STICKS_TO_NULL     = 1 << 2;
        const SELECT_BUTTON      = 1 << 3;
        const PADDLES           = 1 << 4;
        const PROFILE_BUTTON     = 1 << 5;
    }
}

/// Common configuration preset for dance pads
pub const DANCEPAD_MAP_CONFIG: MapFlags = MapFlags::DPAD_TO_BUTTONS
    | MapFlags::TRIGGERS_TO_BUTTONS
    | MapFlags::STICKS_TO_NULL;

/// Xbox controller hardware variants
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XType {
    Xbox = 0,
    Xbox360 = 1,
    Xbox360W = 2,
    XboxOne = 3,
    Unknown = 4,
}

// Power management constants
const XPAD360W_POWEROFF_TIMEOUT: u64 = 5; // Seconds

/// Packet types for different controller protocols
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Xb = 0,
    Xbe1 = 1,
    Xbe2FwOld = 2,
    Xbe2Fw5Early = 3,
    Xbe2Fw511 = 4,
}

bitflags::bitflags! {
    /// Hardware-specific behavior flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct QuirkFlags: u8 {
        const START_PKT_1   = 1 << 0;
        const START_PKT_2   = 1 << 1;
        const START_PKT_3   = 1 << 2;
        const GHL_XBOXONE   = 1 << 3;
    }
}

/// Common quirk combination for Xbox 360 controllers
pub const QUIRK_360_START: QuirkFlags = QuirkFlags::START_PKT_1
    | QuirkFlags::START_PKT_2
    | QuirkFlags::START_PKT_3;

// Module parameters
static DPAD_TO_BUTTONS: AtomicBool = AtomicBool::new(false);
static TRIGGERS_TO_BUTTONS: AtomicBool = AtomicBool::new(false);
static STICKS_TO_NULL: AtomicBool = AtomicBool::new(false);
static AUTO_POWEROFF: AtomicBool = AtomicBool::new(false);

/// Xbox controller device definition
#[derive(Debug, Clone)]
struct XpadDevice {
    id_vendor: u16,
    id_product: u16,
    name: &'static str,
    mapping: MapFlags,
    xtype: XType,
    quirks: QuirkFlags,
}

// Device list using properly defined types
use phf::{phf_ordered_map, OrderedMap};

static XPAD_DEVICES: OrderedMap<(u16, u16), XpadDevice> = phf_ordered_map! {
    (0x0079, 0x18d4) => XpadDevice {
        id_vendor: 0x0079,
        id_product: 0x18d4,
        name: "GPD Win 2 X-Box Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x03eb, 0xff01) => XpadDevice {
        id_vendor: 0x03eb,
        id_product: 0xff01,
        name: "Wooting One (Legacy)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x03eb, 0xff02) => XpadDevice {
        id_vendor: 0x03eb,
        id_product: 0xff02,
        name: "Wooting Two (Legacy)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x038D) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x038D,
        name: "HyperX Clutch",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x048D) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x048D,
        name: "HyperX Clutch",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x0495) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x0495,
        name: "HyperX Clutch Gladiate",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x07A0) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x07A0,
        name: "HyperX Clutch Gladiate RGB",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x08B6) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x08B6,
        name: "HyperX Clutch Gladiate",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x03f0, 0x09B4) => XpadDevice {
        id_vendor: 0x03f0,
        id_product: 0x09B4,
        name: "HyperX Clutch Tanto",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x044f, 0x0f00) => XpadDevice {
        id_vendor: 0x044f,
        id_product: 0x0f00,
        name: "Thrustmaster Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
        (0x044f, 0x0f03) => XpadDevice {
        id_vendor: 0x044f,
        id_product: 0x0f03,
        name: "Thrustmaster Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x044f, 0x0f07) => XpadDevice {
        id_vendor: 0x044f,
        id_product: 0x0f07,
        name: "Thrustmaster, Inc. Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x044f, 0x0f10) => XpadDevice {
        id_vendor: 0x044f,
        id_product: 0x0f10,
        name: "Thrustmaster Modena GT Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x044f, 0xb326) => XpadDevice {
        id_vendor: 0x044f,
        id_product: 0xb326,
        name: "Thrustmaster Gamepad GP XID",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0202) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0202,
        name: "Microsoft X-Box pad v1 (US)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0285) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0285,
        name: "Microsoft X-Box pad (Japan)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0287) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0287,
        name: "Microsoft Xbox Controller S",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0288) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0288,
        name: "Microsoft Xbox Controller S v2",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0289) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0289,
        name: "Microsoft X-Box pad v2 (US)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
        (0x045e, 0x028e) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x028e,
        name: "Microsoft X-Box 360 pad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x028f) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x028f,
        name: "Microsoft X-Box 360 pad v2",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0291) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0291,
        name: "Xbox 360 Wireless Receiver (XBOX)",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360W,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x02a9) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x02a9,
        name: "Xbox 360 Wireless Receiver (Unofficial)",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360W,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x02d1) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x02d1,
        name: "Microsoft X-Box One pad",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x02dd) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x02dd,
        name: "Microsoft X-Box One pad (Firmware 2015)",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x02e3) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x02e3,
        name: "Microsoft X-Box One Elite pad",
        mapping: MapFlags::from_bits(MAP_PADDLES).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x02ea) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x02ea,
        name: "Microsoft X-Box One S pad",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0719) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0719,
        name: "Xbox 360 Wireless Receiver",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360W,
        quirks: QuirkFlags::empty(),
    },
        (0x045e, 0x0b00) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0b00,
        name: "Microsoft X-Box One Elite 2 pad",
        mapping: MapFlags::from_bits(MAP_PADDLES).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0b0a) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0b0a,
        name: "Microsoft X-Box Adaptive Controller",
        mapping: MapFlags::from_bits(MAP_PROFILE_BUTTON).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x045e, 0x0b12) => XpadDevice {
        id_vendor: 0x045e,
        id_product: 0x0b12,
        name: "Microsoft Xbox Series S|X Controller",
        mapping: MapFlags::from_bits(MAP_SELECT_BUTTON).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xc21d) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xc21d,
        name: "Logitech Gamepad F310",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xc21e) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xc21e,
        name: "Logitech Gamepad F510",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xc21f) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xc21f,
        name: "Logitech Gamepad F710",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xc242) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xc242,
        name: "Logitech Chillstream Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xca84) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xca84,
        name: "Logitech Xbox Cordless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xca88) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xca88,
        name: "Logitech Compact Controller for Xbox",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
     (0x046d, 0xca8a) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xca8a,
        name: "Logitech Precision Vibration Feedback Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x046d, 0xcaa3) => XpadDevice {
        id_vendor: 0x046d,
        id_product: 0xcaa3,
        name: "Logitech DriveFx Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x056e, 0x2004) => XpadDevice {
        id_vendor: 0x056e,
        id_product: 0x2004,
        name: "Elecom JC-U3613M",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x05ac, 0x055b) => XpadDevice {
        id_vendor: 0x05ac,
        id_product: 0x055b,
        name: "Gamesir-G3w",
        mapping: MapFlags::from_bits(QUIRK_360_START).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x05fd, 0x1007) => XpadDevice {
        id_vendor: 0x05fd,
        id_product: 0x1007,
        name: "Mad Catz Controller (unverified)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x05fd, 0x107a) => XpadDevice {
        id_vendor: 0x05fd,
        id_product: 0x107a,
        name: "InterAct 'PowerPad Pro' X-Box pad (Germany)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x05fe, 0x3030) => XpadDevice {
        id_vendor: 0x05fe,
        id_product: 0x3030,
        name: "Chic Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x05fe, 0x3031) => XpadDevice {
        id_vendor: 0x05fe,
        id_product: 0x3031,
        name: "Chic Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x062a, 0x0020) => XpadDevice {
        id_vendor: 0x062a,
        id_product: 0x0020,
        name: "Logic3 Xbox GamePad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x062a, 0x0033) => XpadDevice {
        id_vendor: 0x062a,
        id_product: 0x0033,
        name: "Competition Pro Steering Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x06a3, 0x0200) => XpadDevice {
        id_vendor: 0x06a3,
        id_product: 0x0200,
        name: "Saitek Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x06a3, 0x0201) => XpadDevice {
        id_vendor: 0x06a3,
        id_product: 0x0201,
        name: "Saitek Adrenalin",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x06a3, 0xf51a) => XpadDevice {
        id_vendor: 0x06a3,
        id_product: 0xf51a,
        name: "Saitek P3600",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4503) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4503,
        name: "Mad Catz Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4506) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4506,
        name: "Mad Catz 4506 Wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4516) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4516,
        name: "Mad Catz Control Pad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4520) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4520,
        name: "Mad Catz Control Pad Pro",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4522) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4522,
        name: "Mad Catz LumiCON",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4526) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4526,
        name: "Mad Catz Control Pad Pro",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4530) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4530,
        name: "Mad Catz Universal MC2 Racing Wheel and Pedals",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4536) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4536,
        name: "Mad Catz MicroCON",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4540) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4540,
        name: "Mad Catz Beat Pad",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4556) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4556,
        name: "Mad Catz Lynx Wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4586) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4586,
        name: "Mad Catz MicroCon Wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4588) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4588,
        name: "Mad Catz Blaster",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x45ff) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x45ff,
        name: "Mad Catz Beat Pad (w/ Handle)",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4716) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4716,
        name: "Mad Catz Wired Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4718) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4718,
        name: "Mad Catz Street Fighter IV FightStick SE",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4726) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4726,
        name: "Mad Catz Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4728) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4728,
        name: "Mad Catz Street Fighter IV FightPad",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4736) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4736,
        name: "Mad Catz MicroCon Gamepad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4738) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4738,
        name: "Mad Catz Wired Xbox 360 Controller (SFIV)",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4740) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4740,
        name: "Mad Catz Beat Pad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4743) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4743,
        name: "Mad Catz Beat Pad Pro",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4758) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4758,
        name: "Mad Catz Arcade Game Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x4a01) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x4a01,
        name: "Mad Catz FightStick TE 2",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x6040) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x6040,
        name: "Mad Catz Beat Pad Pro",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0x9871) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0x9871,
        name: "Mad Catz Portable Drum",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xb726) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xb726,
        name: "Mad Catz Xbox controller - MW2",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xb738) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xb738,
        name: "Mad Catz MVC2TE Stick 2",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xbeef) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xbeef,
        name: "Mad Catz JOYTECH NEO SE Advanced GamePad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xcb02) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xcb02,
        name: "Saitek Cyborg Rumble Pad - PC/Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xcb03) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xcb03,
        name: "Saitek P3200 Rumble Pad - PC/Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xcb29) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xcb29,
        name: "Saitek Aviator Stick AV8R02",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0738, 0xf738) => XpadDevice {
        id_vendor: 0x0738,
        id_product: 0xf738,
        name: "Super SFIV FightStick TE S",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x07ff, 0xffff) => XpadDevice {
        id_vendor: 0x07ff,
        id_product: 0xffff,
        name: "Mad Catz GamePad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0b05, 0x1a38) => XpadDevice {
        id_vendor: 0x0b05,
        id_product: 0x1a38,
        name: "ASUS ROG RAIKIRI",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0b05, 0x1abb) => XpadDevice {
        id_vendor: 0x0b05,
        id_product: 0x1abb,
        name: "ASUS ROG RAIKIRI PRO",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x0005) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x0005,
        name: "Intec wireless",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x8801) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x8801,
        name: "Nyko Xbox Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x8802) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x8802,
        name: "Zeroplus Xbox Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x8809) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x8809,
        name: "RedOctane Xbox Dance Pad",
        mapping: MapFlags::from_bits(DANCEPAD_MAP_CONFIG).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x880a) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x880a,
        name: "Pelican Eclipse PL-2023",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x8810) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x8810,
        name: "Zeroplus Xbox Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0c12, 0x9902) => XpadDevice {
        id_vendor: 0x0c12,
        id_product: 0x9902,
        name: "HAMA VibraX - *FAULTY HARDWARE*",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0d2f, 0x0002) => XpadDevice {
        id_vendor: 0x0d2f,
        id_product: 0x0002,
        name: "Andamiro Pump It Up pad",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0db0, 0x1901) => XpadDevice {
        id_vendor: 0x0db0,
        id_product: 0x1901,
        name: "Micro Star International Xbox360 Controller for Windows",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e4c, 0x1097) => XpadDevice {
        id_vendor: 0x0e4c,
        id_product: 0x1097,
        name: "Radica Gamester Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e4c, 0x1103) => XpadDevice {
        id_vendor: 0x0e4c,
        id_product: 0x1103,
        name: "Radica Gamester Reflex",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e4c, 0x2390) => XpadDevice {
        id_vendor: 0x0e4c,
        id_product: 0x2390,
        name: "Radica Games Jtech Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e4c, 0x3510) => XpadDevice {
        id_vendor: 0x0e4c,
        id_product: 0x3510,
        name: "Radica Gamester",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0003) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0003,
        name: "Logic3 Freebird wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0005) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0005,
        name: "Eclipse wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0006) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0006,
        name: "Edge wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0008) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0008,
        name: "After Glow Pro Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0105) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0105,
        name: "HSM3 Xbox360 dancepad",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0113) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0113,
        name: "Afterglow AX.1 Gamepad for Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x011f) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x011f,
        name: "Rock Candy Gamepad Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0131) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0131,
        name: "PDP EA Sports Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0133) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0133,
        name: "Xbox 360 Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0139) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0139,
        name: "Afterglow Prismatic Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x013a) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x013a,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0146) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0146,
        name: "Rock Candy Wired Controller for Xbox One",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0147) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0147,
        name: "PDP Marvel Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x015c) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x015c,
        name: "PDP Xbox One Arcade Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x015d) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x015d,
        name: "PDP Mirror's Edge Official Wired Controller for Xbox One",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0161) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0161,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0162) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0162,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0163) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0163,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0164) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0164,
        name: "PDP Battlefield One",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0165) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0165,
        name: "PDP Titanfall 2",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
        (0x0e6f, 0x0201) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0201,
        name: "Pelican PL-3601 'TSZ' Wired Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0213) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0213,
        name: "Afterglow Gamepad for Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x021f) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x021f,
        name: "Rock Candy Gamepad for Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0246) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0246,
        name: "Rock Candy Gamepad for Xbox One 2015",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a0) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a0,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a1) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a1,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a2) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a2,
        name: "PDP Wired Controller for Xbox One - Crimson Red",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a4) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a4,
        name: "PDP Wired Controller for Xbox One - Stealth Series",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a6) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a6,
        name: "PDP Wired Controller for Xbox One - Camo Series",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a7) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a7,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02a8) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02a8,
        name: "PDP Xbox One Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02ab) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02ab,
        name: "PDP Controller for Xbox One",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02ad) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02ad,
        name: "PDP Wired Controller for Xbox One - Stealth Series",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02b3) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02b3,
        name: "Afterglow Prismatic Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x02b8) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x02b8,
        name: "Afterglow Prismatic Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0301) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0301,
        name: "Logic3 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0346) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0346,
        name: "Rock Candy Gamepad for Xbox One 2016",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0401) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0401,
        name: "Logic3 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0413) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0413,
        name: "Afterglow AX.1 Gamepad for Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0x0501) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0x0501,
        name: "PDP Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e6f, 0xf900) => XpadDevice {
        id_vendor: 0x0e6f,
        id_product: 0xf900,
        name: "PDP Afterglow AX.1",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0e8f, 0x0201) => XpadDevice {
        id_vendor: 0x0e8f,
        id_product: 0x0201,
        name: "SmartJoy Frag Xpad/PS2 adaptor",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0e8f, 0x3008) => XpadDevice {
        id_vendor: 0x0e8f,
        id_product: 0x3008,
        name: "Generic xbox control (dealextreme)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x000a) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x000a,
        name: "Hori Co. DOA4 FightStick",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x000c) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x000c,
        name: "Hori PadEX Turbo",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x000d) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x000d,
        name: "Hori Fighting Stick EX2",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0016) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0016,
        name: "Hori Real Arcade Pro.EX",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x001b) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x001b,
        name: "Hori Real Arcade Pro VX",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0063) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0063,
        name: "Hori Real Arcade Pro Hayabusa (USA) Xbox One",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0067) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0067,
        name: "HORIPAD ONE",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0078) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0078,
        name: "Hori Real Arcade Pro V Kai Xbox One",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x00c5) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x00c5,
        name: "Hori Fighting Commander ONE",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x00dc) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x00dc,
        name: "HORIPAD FPS for Nintendo Switch",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0152) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0152,
        name: "Hori Racing Wheel Overdrive for Xbox Series X",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f0d, 0x0151) => XpadDevice {
        id_vendor: 0x0f0d,
        id_product: 0x0151,
        name: "Hori Racing Wheel Overdrive for Xbox Series X",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x0f30, 0x010b) => XpadDevice {
        id_vendor: 0x0f30,
        id_product: 0x010b,
        name: "Philips Recoil",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0f30, 0x0202) => XpadDevice {
        id_vendor: 0x0f30,
        id_product: 0x0202,
        name: "Joytech Advanced Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0f30, 0x8888) => XpadDevice {
        id_vendor: 0x0f30,
        id_product: 0x8888,
        name: "BigBen XBMiniPad Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x102c, 0xff0c) => XpadDevice {
        id_vendor: 0x102c,
        id_product: 0xff0c,
        name: "Joytech Wireless Advanced Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x1038, 0x1430) => XpadDevice {
        id_vendor: 0x1038,
        id_product: 0x1430,
        name: "SteelSeries Stratus Duo",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1038, 0x1431) => XpadDevice {
        id_vendor: 0x1038,
        id_product: 0x1431,
        name: "SteelSeries Stratus Duo",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x10f5, 0x7005) => XpadDevice {
        id_vendor: 0x10f5,
        id_product: 0x7005,
        name: "Turtle Beach Recon Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x11c9, 0x55f0) => XpadDevice {
        id_vendor: 0x11c9,
        id_product: 0x55f0,
        name: "Nacon GC-100XF",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x11ff, 0x0511) => XpadDevice {
        id_vendor: 0x11ff,
        id_product: 0x0511,
        name: "PXN V900",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1209, 0x2882) => XpadDevice {
        id_vendor: 0x1209,
        id_product: 0x2882,
        name: "Ardwiino Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x12ab, 0x0004) => XpadDevice {
        id_vendor: 0x12ab,
        id_product: 0x0004,
        name: "Honey Bee Xbox360 dancepad",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
        (0x12ab, 0x0301) => XpadDevice {
        id_vendor: 0x12ab,
        id_product: 0x0301,
        name: "PDP AFTERGLOW AX.1",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x12ab, 0x0303) => XpadDevice {
        id_vendor: 0x12ab,
        id_product: 0x0303,
        name: "Mortal Kombat Klassic FightStick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x12ab, 0x8809) => XpadDevice {
        id_vendor: 0x12ab,
        id_product: 0x8809,
        name: "Xbox DDR dancepad",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x1430, 0x079B) => XpadDevice {
        id_vendor: 0x1430,
        id_product: 0x079B,
        name: "RedOctane GHL Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::from_bits(QUIRK_GHL_XBOXONE).unwrap(),
    },
    (0x1430, 0x4748) => XpadDevice {
        id_vendor: 0x1430,
        id_product: 0x4748,
        name: "RedOctane Guitar Hero X-plorer",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1430, 0x8888) => XpadDevice {
        id_vendor: 0x1430,
        id_product: 0x8888,
        name: "TX6500+ Dance Pad (first generation)",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x1430, 0xf801) => XpadDevice {
        id_vendor: 0x1430,
        id_product: 0xf801,
        name: "RedOctane Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x146b, 0x0601) => XpadDevice {
        id_vendor: 0x146b,
        id_product: 0x0601,
        name: "BigBen Interactive XBOX 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x146b, 0x0604) => XpadDevice {
        id_vendor: 0x146b,
        id_product: 0x0604,
        name: "Bigben Interactive DAIJA Arcade Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1532, 0x0a00) => XpadDevice {
        id_vendor: 0x1532,
        id_product: 0x0a00,
        name: "Razer Atrox Arcade Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x1532, 0x0a03) => XpadDevice {
        id_vendor: 0x1532,
        id_product: 0x0a03,
        name: "Razer Wildcat",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x1532, 0x0a29) => XpadDevice {
        id_vendor: 0x1532,
        id_product: 0x0a29,
        name: "Razer Wolverine V2",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x15e4, 0x3f00) => XpadDevice {
        id_vendor: 0x15e4,
        id_product: 0x3f00,
        name: "Power A Mini Pro Elite",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x15e4, 0x3f0a) => XpadDevice {
        id_vendor: 0x15e4,
        id_product: 0x3f0a,
        name: "Xbox Airflo wired controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x15e4, 0x3f10) => XpadDevice {
        id_vendor: 0x15e4,
        id_product: 0x3f10,
        name: "Batarang Xbox 360 controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x162e, 0xbeef) => XpadDevice {
        id_vendor: 0x162e,
        id_product: 0xbeef,
        name: "Joytech Neo-Se Take2",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1689, 0xfd00) => XpadDevice {
        id_vendor: 0x1689,
        id_product: 0xfd00,
        name: "Razer Onza Tournament Edition",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1689, 0xfd01) => XpadDevice {
        id_vendor: 0x1689,
        id_product: 0xfd01,
        name: "Razer Onza Classic Edition",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1689, 0xfe00) => XpadDevice {
        id_vendor: 0x1689,
        id_product: 0xfe00,
        name: "Razer Sabertooth",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x17ef, 0x6182) => XpadDevice {
        id_vendor: 0x17ef,
        id_product: 0x6182,
        name: "Lenovo Legion Controller for Windows",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1949, 0x041a) => XpadDevice {
        id_vendor: 0x1949,
        id_product: 0x041a,
        name: "Amazon Game Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1a86, 0xe310) => XpadDevice {
        id_vendor: 0x1a86,
        id_product: 0xe310,
        name: "QH Electronics Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0x0002) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0x0002,
        name: "Harmonix Rock Band Guitar",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0x0003) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0x0003,
        name: "Harmonix Rock Band Drumkit",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0x0130) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0x0130,
        name: "Ion Drum Rocker",
        mapping: MapFlags::from_bits(MAP_DPAD_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf016) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf016,
        name: "Mad Catz Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf018) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf018,
        name: "Mad Catz Street Fighter IV SE Fighting Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf019) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf019,
        name: "Mad Catz Brawlstick for Xbox 360",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf021) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf021,
        name: "Mad Cats Ghost Recon FS GamePad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf023) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf023,
        name: "MLG Pro Circuit Controller (Xbox)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf025) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf025,
        name: "Mad Catz Call Of Duty",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf027) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf027,
        name: "Mad Catz FPS Pro",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf028) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf028,
        name: "Street Fighter IV FightPad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf02e) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf02e,
        name: "Mad Catz Fightpad",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf030) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf030,
        name: "Mad Catz Xbox 360 MC2 MicroCon Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf036) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf036,
        name: "Mad Catz MicroCon GamePad Pro",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf038) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf038,
        name: "Street Fighter IV FightStick TE",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf039) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf039,
        name: "Mad Catz MvC2 TE",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf03a) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf03a,
        name: "Mad Catz SFxT Fightstick Pro",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf03d) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf03d,
        name: "Street Fighter IV Arcade Stick TE - Chun Li",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf03e) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf03e,
        name: "Mad Catz MLG FightStick TE",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf03f) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf03f,
        name: "Mad Catz FightStick SoulCaliber",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf042) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf042,
        name: "Mad Catz FightStick TES+",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf080) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf080,
        name: "Mad Catz FightStick TE2",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf501) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf501,
        name: "HoriPad EX2 Turbo",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf502) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf502,
        name: "Hori Real Arcade Pro.VX SA",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf503) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf503,
        name: "Hori Fighting Stick VX",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf504) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf504,
        name: "Hori Real Arcade Pro. EX",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
        (0x1bad, 0xf505) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf505,
        name: "Hori Fighting Stick EX2B",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf506) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf506,
        name: "Hori Real Arcade Pro.EX Premium VLX",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf900) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf900,
        name: "Harmonix Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf901) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf901,
        name: "Gamestop Xbox 360 Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf903) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf903,
        name: "Tron Xbox 360 controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf904) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf904,
        name: "PDP Versus Fighting Pad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xf906) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xf906,
        name: "Mortal Kombat FightStick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xfa01) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xfa01,
        name: "MadCatz GamePad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xfd00) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xfd00,
        name: "Razer Onza TE",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x1bad, 0xfd01) => XpadDevice {
        id_vendor: 0x1bad,
        id_product: 0xfd01,
        name: "Razer Onza",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x20d6, 0x2001) => XpadDevice {
        id_vendor: 0x20d6,
        id_product: 0x2001,
        name: "BDA Xbox Series X Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x20d6, 0x2009) => XpadDevice {
        id_vendor: 0x20d6,
        id_product: 0x2009,
        name: "PowerA Enhanced Wired Controller for Xbox Series X|S",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x20d6, 0x281f) => XpadDevice {
        id_vendor: 0x20d6,
        id_product: 0x281f,
        name: "PowerA Wired Controller For Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x2345, 0xe00b) => XpadDevice {
        id_vendor: 0x2345,
        id_product: 0xe00b,
        name: "Machenike G5 Pro Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5000) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5000,
        name: "Razer Atrox Arcade Stick",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5300) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5300,
        name: "PowerA MINI PROEX Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5303) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5303,
        name: "Xbox Airflo wired controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x530a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x530a,
        name: "Xbox 360 Pro EX Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x531a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x531a,
        name: "PowerA Pro Ex",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5397) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5397,
        name: "FUS1ON Tournament Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x541a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x541a,
        name: "PowerA Xbox One Mini Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x542a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x542a,
        name: "Xbox ONE spectra",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x543a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x543a,
        name: "PowerA Xbox One wired controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5500) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5500,
        name: "Hori XBOX 360 EX 2 with Turbo",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5501) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5501,
        name: "Hori Real Arcade Pro VX-SA",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5502) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5502,
        name: "Hori Fighting Stick VX Alt",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5503) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5503,
        name: "Hori Fighting Edge",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5506) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5506,
        name: "Hori SOULCALIBUR V Stick",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x550d) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x550d,
        name: "Hori GEM Xbox controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x550e) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x550e,
        name: "Hori Real Arcade Pro V Kai 360",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5510) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5510,
        name: "Hori Fighting Commander ONE (Xbox 360/PC Mode)",
        mapping: MapFlags::from_bits(MAP_TRIGGERS_TO_BUTTONS).unwrap(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x551a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x551a,
        name: "PowerA FUSION Pro Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x561a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x561a,
        name: "PowerA FUSION Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x581a) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x581a,
        name: "ThrustMaster XB1 Classic Controller",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5b00) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5b00,
        name: "ThrustMaster Ferrari 458 Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5b02) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5b02,
        name: "Thrustmaster, Inc. GPX Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5b03) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5b03,
        name: "Thrustmaster Ferrari 458 Racing Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0x5d04) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0x5d04,
        name: "Razer Sabertooth",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x24c6, 0xfafe) => XpadDevice {
        id_vendor: 0x24c6,
        id_product: 0xfafe,
        name: "Rock Candy Gamepad for Xbox 360",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x2563, 0x058d) => XpadDevice {
        id_vendor: 0x2563,
        id_product: 0x058d,
        name: "OneXPlayer Gamepad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x294b, 0x3303) => XpadDevice {
        id_vendor: 0x294b,
        id_product: 0x3303,
        name: "Snakebyte GAMEPAD BASE X",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x294b, 0x3404) => XpadDevice {
        id_vendor: 0x294b,
        id_product: 0x3404,
        name: "Snakebyte GAMEPAD RGB X",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x2dc8, 0x2000) => XpadDevice {
        id_vendor: 0x2dc8,
        id_product: 0x2000,
        name: "8BitDo Pro 2 Wired Controller fox Xbox",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x2dc8, 0x3106) => XpadDevice {
        id_vendor: 0x2dc8,
        id_product: 0x3106,
        name: "8BitDo Ultimate Wireless / Pro 2 Wired Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x2dc8, 0x3109) => XpadDevice {
        id_vendor: 0x2dc8,
        id_product: 0x3109,
        name: "8BitDo Ultimate Wireless Bluetooth",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x2dc8, 0x310a) => XpadDevice {
        id_vendor: 0x2dc8,
        id_product: 0x310a,
        name: "8BitDo Ultimate 2C Wireless Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x2e24, 0x0652) => XpadDevice {
        id_vendor: 0x2e24,
        id_product: 0x0652,
        name: "Hyperkin Duke X-Box One pad",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x2e95, 0x0504) => XpadDevice {
        id_vendor: 0x2e95,
        id_product: 0x0504,
        name: "SCUF Gaming Controller",
        mapping: MapFlags::from_bits(MAP_SELECT_BUTTON).unwrap(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1100) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1100,
        name: "Wooting One",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1200) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1200,
        name: "Wooting Two",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1210) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1210,
        name: "Wooting Lekker",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1220) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1220,
        name: "Wooting Two HE",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1230) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1230,
        name: "Wooting Two HE (ARM)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1300) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1300,
        name: "Wooting 60HE (AVR)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x31e3, 0x1310) => XpadDevice {
        id_vendor: 0x31e3,
        id_product: 0x1310,
        name: "Wooting 60HE (ARM)",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x3285, 0x0603) => XpadDevice {
        id_vendor: 0x3285,
        id_product: 0x0603,
        name: "Nacon Pro Compact controller for Xbox",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x3285, 0x0607) => XpadDevice {
        id_vendor: 0x3285,
        id_product: 0x0607,
        name: "Nacon GC-100",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x3285, 0x0614) => XpadDevice {
        id_vendor: 0x3285,
        id_product: 0x0614,
        name: "Nacon Pro Compact",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x3285, 0x0662) => XpadDevice {
        id_vendor: 0x3285,
        id_product: 0x0662,
        name: "Nacon Revolution5 Pro",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x3285, 0x0663) => XpadDevice {
        id_vendor: 0x3285,
        id_product: 0x0663,
        name: "Nacon Evol-X",
        mapping: MapFlags::empty(),
        xtype: XType::XboxOne,
        quirks: QuirkFlags::empty(),
    },
    (0x3537, 0x1004) => XpadDevice {
        id_vendor: 0x3537,
        id_product: 0x1004,
        name: "GameSir T4 Kaleid",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0x3767, 0x0101) => XpadDevice {
        id_vendor: 0x3767,
        id_product: 0x0101,
        name: "Fanatec Speedster 3 Forceshock Wheel",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x413d, 0x2104) => XpadDevice {
        id_vendor: 0x413d,
        id_product: 0x2104,
        name: "Black Shark Green Ghost Gamepad",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox360,
        quirks: QuirkFlags::empty(),
    },
    (0xffff, 0xffff) => XpadDevice {
        id_vendor: 0xffff,
        id_product: 0xffff,
        name: "Chinese-made Xbox Controller",
        mapping: MapFlags::empty(),
        xtype: XType::Xbox,
        quirks: QuirkFlags::empty(),
    },
    (0x0000, 0x0000) => XpadDevice {
        id_vendor: 0x0000,
        id_product: 0x0000,
        name: "Generic X-Box pad",
        mapping: MapFlags::empty(),
        xtype: XType::Unknown,
        quirks: QuirkFlags::empty(),
    },
};

// buttons shared with xbox and xbox360
const XPAD_COMMON_BTN: [i16; 9] = [
    BTN_A, BTN_B, BTN_X, BTN_Y,            // "analog" buttons
    BTN_START, BTN_SELECT, BTN_THUMBL, BTN_THUMBR,  // start/back/sticks
    -1                                     // terminating entry
];

// original xbox controllers only
const XPAD_BTN: [i16; 3] = [
    BTN_C, BTN_Z,        // "analog" buttons
    -1                   // terminating entry
];

// used when dpad is mapped to buttons
const XPAD_BTN_PAD: [i16; 5] = [
    BTN_TRIGGER_HAPPY1, BTN_TRIGGER_HAPPY2,     // d-pad left, right
    BTN_TRIGGER_HAPPY3, BTN_TRIGGER_HAPPY4,     // d-pad up, down
    -1                         // terminating entry
];

// used when triggers are mapped to buttons
const XPAD_BTN_TRIGGERS: [i16; 3] = [
    BTN_TL2, BTN_TR2,        // triggers left/right
    -1
];

// buttons for x360 controller
const XPAD360_BTN: [i16; 4] = [
    BTN_TL, BTN_TR,        // Button LB/RB
    BTN_MODE,              // The big X button
    -1
];

const XPAD_ABS: [i16; 5] = [
    ABS_X, ABS_Y,        // left stick
    ABS_RX, ABS_RY,      // right stick
    -1                   // terminating entry
];

// used when dpad is mapped to axes
const XPAD_ABS_PAD: [i16; 3] = [
    ABS_HAT0X, ABS_HAT0Y,  // d-pad axes
    -1                     // terminating entry
];

// used when triggers are mapped to axes
const XPAD_ABS_TRIGGERS: [i16; 3] = [
    ABS_Z, ABS_RZ,        // triggers left/right
    -1
];

// used when the controller has extra paddle buttons
const XPAD_BTN_PADDLES: [i16; 5] = [
    BTN_TRIGGER_HAPPY5, BTN_TRIGGER_HAPPY6,  // paddle upper right, lower right
    BTN_TRIGGER_HAPPY7, BTN_TRIGGER_HAPPY8,  // paddle upper left, lower left
    -1                                      // terminating entry
];

// used for GHL dpad mapping
const DPAD_MAPPING: [(i16, i16); 9] = [
    (0, -1), (1, -1), (1, 0), (1, 1),
    (0, 1), (-1, 1), (-1, 0), (-1, -1),
    (0, 0)
];

// USB constants and device matching logic
mod linux_usb {
    pub const USB_CLASS_VENDOR_SPEC: u8 = 0xff;
    pub const USB_DEVICE_ID_MATCH_VENDOR: u16 = 0x0001;
    pub const USB_DEVICE_ID_MATCH_INT_INFO: u16 = 0x0002;
}

#[derive(Debug, Clone, Copy)]
struct UsbDeviceId {
    match_flags: u16,
    id_vendor: u16,
    b_interface_class: u8,
    b_interface_subclass: u8,
    b_interface_protocol: u8,
}

impl UsbDeviceId {
    const fn xbox360_vendor_proto(vend: u16, pr: u8) -> Self {
        Self {
            match_flags: linux_usb::USB_DEVICE_ID_MATCH_VENDOR 
                       | linux_usb::USB_DEVICE_ID_MATCH_INT_INFO,
            id_vendor: vend,
            b_interface_class: linux_usb::USB_CLASS_VENDOR_SPEC,
            b_interface_subclass: 93,
            b_interface_protocol: pr,
        }
    }

    const fn xboxone_vendor_proto(vend: u16, pr: u8) -> Self {
        Self {
            match_flags: linux_usb::USB_DEVICE_ID_MATCH_VENDOR 
                       | linux_usb::USB_DEVICE_ID_MATCH_INT_INFO,
            id_vendor: vend,
            b_interface_class: linux_usb::USB_CLASS_VENDOR_SPEC,
            b_interface_subclass: 71,
            b_interface_protocol: pr,
        }
    }
}

const XPAD_TABLE: &[UsbDeviceId] = &[
    // Original Xbox controller
    UsbDeviceId {
        match_flags: linux_usb::USB_DEVICE_ID_MATCH_INT_INFO,
        id_vendor: 0,
        b_interface_class: b'X',
        b_interface_subclass: b'B',
        b_interface_protocol: 0,
    },
    // GPD Win 2 controller (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0079)[0],
    UsbDeviceId::xbox360_vendor(0x0079)[1],

    // Wooting Keyboards (expanded safely)
    UsbDeviceId::xbox360_vendor(0x03eb)[0],
    UsbDeviceId::xbox360_vendor(0x03eb)[1],

    // HP HyperX Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x03f0)[0],
    UsbDeviceId::xbox360_vendor(0x03f0)[1],

    // HP HyperX Xbox One controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x03f0)[0],

    // Thrustmaster Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x044f)[0],
    UsbDeviceId::xbox360_vendor(0x044f)[1],

    // Thrustmaster Xbox One controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x044f)[0],

    // Microsoft Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x045e)[0],
    UsbDeviceId::xbox360_vendor(0x045e)[1],

    // Microsoft Xbox One controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x045e)[0],

    // Logitech Xbox 360-style controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x046d)[0],
    UsbDeviceId::xbox360_vendor(0x046d)[1],

    // Elecom JC-U3613M (expanded safely)
    UsbDeviceId::xbox360_vendor(0x056e)[0],

    // Saitek P3600 (expanded safely)
    UsbDeviceId::xbox360_vendor(0x06a3)[0],

    // Mad Catz Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0738)[0],

    // Mad Catz Beat Pad (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0738)[1],

    // Mad Catz FightStick TE 2 (expanded safely)
    UsbDeviceId::xboxone_vendor(0x0738)[0],

    // Mad Catz Gamepad (expanded safely)
    UsbDeviceId::xbox360_vendor(0x07ff)[0],

    // ASUS controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x0b05)[0],

    // Zeroplus X-Box 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0c12)[0],

    // Micro Star International X-Box 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0db0)[0],

    // 0x0e6f Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0e6f)[0],

    // 0x0e6f Xbox One controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x0e6f)[0],

    // Hori controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x0f0d)[0],
    UsbDeviceId::xboxone_vendor(0x0f0d)[0],

    // SteelSeries controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1038)[0],

    // Turtle Beach Controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x10f5)[0],

    // Nacon GC100XF (expanded safely)
    UsbDeviceId::xbox360_vendor(0x11c9)[0],

    // PXN V900 (expanded safely)
    UsbDeviceId::xbox360_vendor(0x11ff)[0],

    // Ardwiino Controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1209)[0],

    // Xbox 360 dance pads (expanded safely)
    UsbDeviceId::xbox360_vendor(0x12ab)[0],

    // RedOctane Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1430)[0],

    // RedOctane X-Box One controllers (expanded safely)
    UsbDeviceId::xboxone_vendor(0x1430)[0],

    // Bigben Interactive controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x146b)[0],

    // Razer Sabertooth (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1532)[0],

    // Razer Wildcat (expanded safely)
    UsbDeviceId::xboxone_vendor(0x1532)[0],

    // Numark Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x15e4)[0],

    // Joytech Xbox 360 controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x162e)[0],

    // Razer Onza (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1689)[0],

    // Lenovo (expanded safely)
    UsbDeviceId::xbox360_vendor(0x17ef)[0],

    // Amazon controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1949)[0],

    // QH Electronics (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1a86)[0],

    // Harmonix Rock Band guitar and drums (expanded safely)
    UsbDeviceId::xbox360_vendor(0x1bad)[0],
    UsbDeviceId::xbox360_vendor(0x1bad)[1],

    // PowerA controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x20d6)[0],
    UsbDeviceId::xboxone_vendor(0x20d6)[0],

    // Machenike Controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x2345)[0],

    // PowerA controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x24c6)[0],
    UsbDeviceId::xboxone_vendor(0x24c6)[0],

    // OneXPlayer Gamepad (expanded safely)
    UsbDeviceId::xbox360_vendor(0x2563)[0],

    // Dareu H101 (expanded safely)
    UsbDeviceId::xbox360_vendor(0x260d)[0],

    // Snakebyte (expanded safely)
    UsbDeviceId::xboxone_vendor(0x294b)[0],

    // Qanba Controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x2c22)[0],

    // 8BitDo Pro 2 Wired Controller (expanded safely)
    UsbDeviceId::xbox360_vendor(0x2dc8)[0],

    // 8BitDo Pro 2 Wired Controller for Xbox (expanded safely)
    UsbDeviceId::xboxone_vendor(0x2dc8)[0],

    // Hyperkin Duke Xbox One pad (expanded safely)
    UsbDeviceId::xboxone_vendor(0x2e24)[0],

    // SCUF Gaming Controller (expanded safely)
    UsbDeviceId::xboxone_vendor(0x2e95)[0],

    // Wooting Keyboards (expanded safely)
    UsbDeviceId::xbox360_vendor(0x31e3)[0],

    // Nacon GC-100 (expanded safely)
    UsbDeviceId::xbox360_vendor(0x3285)[0],

    // Nacon Evol-X (expanded safely)
    UsbDeviceId::xboxone_vendor(0x3285)[0],

    // GameSir Controllers (expanded safely)
    UsbDeviceId::xbox360_vendor(0x3537)[0],
    UsbDeviceId::xboxone_vendor(0x3537)[0],

    // Black Shark Green Ghost Controller (expanded safely)
    UsbDeviceId::xbox360_vendor(0x413d)[0],
];


// Improved initialization with error handling
fn init_devices() -> kernel::Result {
    for device in XPAD_DEVICES.values() {
        kernel::pr_info!(
            "Initializing {:04x}:{:04x} - {}",
            device.id_vendor,
            device.id_product,
            device.name
        );
        
        // Safe hardware access in unsafe block
        unsafe {
            send_control_transfer(device, INIT_PACKETS)?;
        }
    }
    Ok(())
}

// Enhanced packet processing with proper error handling
fn process_packet(dev: &mut InputDev, cmd: u16, data: &[u8]) -> Result<(), kernel::Error> {
    if data.len() < XPAD_PKT_LEN {
        return Err(kernel::Error::EINVAL);
    }

    // Validate and process packet data
    let buttons = data[2];
    let triggers = (data[10], data[11]);
    
    // Process analog sticks
    if !STICKS_TO_NULL.load(Ordering::Relaxed) {
        let x = i16::from_le_bytes([data[12], data[13]]);
        let y = i16::from_le_bytes([data[14], data[15]]);
        input_report_abs(dev, ABS_X, x.into());
        input_report_abs(dev, ABS_Y, (!y).into());
    }

    // Process triggers
    if TRIGGERS_TO_BUTTONS.load(Ordering::Relaxed) {
        input_report_key(dev, BTN_TL2, triggers.0 > 0);
        input_report_key(dev, BTN_TR2, triggers.1 > 0);
    } else {
        input_report_abs(dev, ABS_Z, triggers.0.into());
        input_report_abs(dev, ABS_RZ, triggers.1.into());
    }

    // Process D-pad
    if DPAD_TO_BUTTONS.load(Ordering::Relaxed) {
        input_report_key(dev, BTN_TRIGGER_HAPPY1, buttons & 0x04 != 0);
        input_report_key(dev, BTN_TRIGGER_HAPPY2, buttons & 0x08 != 0);
        input_report_key(dev, BTN_TRIGGER_HAPPY3, buttons & 0x01 != 0);
        input_report_key(dev, BTN_TRIGGER_HAPPY4, buttons & 0x02 != 0);
    } else {
        let hat_x = (buttons & 0x04 != 0) as i32 - (buttons & 0x08 != 0) as i32;
        let hat_y = (buttons & 0x01 != 0) as i32 - (buttons & 0x02 != 0) as i32;
        input_report_abs(dev, ABS_HAT0X, hat_x);
        input_report_abs(dev, ABS_HAT0Y, hat_y);
    }

    input_sync(dev);
    Ok(())
}

/*
 * xpad360w_process_packet
 *
 * Completes a request by converting the data into events for the
 * input subsystem. It is version for xbox 360 wireless controller.
 *
 * Byte.Bit
 * 00.1 - Status change: The controller or headset has connected/disconnected
 *                       Bits 01.7 and 01.6 are valid
 * 01.7 - Controller present
 * 01.6 - Headset present
 * 01.1 - Pad state (Bytes 4+) valid
 *
 */
use std::rc::Rc;

struct UsbXpad {
    pad_present: bool,
    x360w_dev: Option<Rc<input_dev>>,
}

impl UsbXpad {
    fn process_packet(&mut self, cmd: u16, data: &[u8]) {
        if data[0] & 0x08 != 0 {
            let present = data[1] & 0x80 != 0;
            if self.pad_present != present {
                self.pad_present = present;
                self.schedule_work();
            }
        } else if data[1] == 0x1 {
            let x360w_dev = Rc::new(input_dev);
            if let Some(ref mut dev) = self.x360w_dev {
                xpad360_process_packet(self, dev, cmd, &data[4]);
            }
        }
    }
}

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use usb::{Urb, UsbDevice, UsbError};
use input::{InputDevice, InputEvent, AbsoluteAxis, Key, Button};

// Shared state structure
struct UsbXpad {
    xtype: XType,
    dev: Arc<InputDevice>,
    pad_present: AtomicBool,
    irq_out_active: AtomicBool,
    odata: Mutex<Vec<u8>>,
    init_seq: Mutex<usize>,
    mapping: MapFlags,
    packet_type: PacketType,
    quirks: QuirkFlags,
}

#[derive(Debug, Clone, Copy)]
enum XType {
    Xbox360,
    Xbox360W,
    XboxOne,
    Unknown,
}

// Xbox 360 Wireless packet processing
fn xpad360w_process_packet(xpad: &UsbXpad, data: &[u8]) {
    // Check presence change
    if data[0] & 0x08 != 0 {
        let present = data[1] & 0x80 != 0;
        if xpad.pad_present.swap(present, Ordering::SeqCst) != present {
            // Schedule work for presence change
            // (Would typically use a channel or async task here)
        }
    }

    // Process valid pad data
    if data[1] == 0x01 && data.len() >= 4 {
        let dev = xpad.dev.clone();
        xpad360_process_packet(&dev, &data[4..]);
    }
}

// Xbox One packet processing
fn xpadone_process_packet(xpad: &UsbXpad, data: &[u8]) {
    let dev = xpad.dev.clone();
    let mut do_sync = false;

    match data[0] {
        GIP_CMD_VIRTUAL_KEY => {
            if data[1] == (GIP_OPT_ACK | GIP_OPT_INTERNAL) {
                xpadone_ack_mode_report(xpad, data[2]);
            }
            dev.report_key(Button::Mode, data[4] & 0x03 != 0);
            do_sync = true;
        },
        GIP_CMD_FIRMWARE => {
            if xpad.packet_type == PacketType::Xbe2Fw5_11 {
                let buttons = if data[19] != 0 { 0 } else { data[18] };
                dev.report_key(Button::TriggerHappy5, buttons & 0x01 != 0);
                dev.report_key(Button::TriggerHappy6, buttons & 0x02 != 0);
                dev.report_key(Button::TriggerHappy7, buttons & 0x04 != 0);
                dev.report_key(Button::TriggerHappy8, buttons & 0x08 != 0);
                do_sync = true;
            }
        },
        GIP_CMD_INPUT => {
            // Main input processing
            dev.report_key(Button::Start, data[4] & 0x04 != 0);
            dev.report_key(Button::Select, data[4] & 0x08 != 0);
            
            // Buttons
            dev.report_key(Button::A, data[4] & 0x10 != 0);
            dev.report_key(Button::B, data[4] & 0x20 != 0);
            dev.report_key(Button::X, data[4] & 0x40 != 0);
            dev.report_key(Button::Y, data[4] & 0x80 != 0);

            // D-pad handling
            if xpad.mapping.contains(MapFlags::DPAD_TO_BUTTONS) {
                dev.report_key(Button::TriggerHappy1, data[5] & 0x04 != 0);
                dev.report_key(Button::TriggerHappy2, data[5] & 0x08 != 0);
                dev.report_key(Button::TriggerHappy3, data[5] & 0x01 != 0);
                dev.report_key(Button::TriggerHappy4, data[5] & 0x02 != 0);
            } else {
                let hat_x = (data[5] & 0x08 != 0) as i32 - (data[5] & 0x04 != 0) as i32;
                let hat_y = (data[5] & 0x02 != 0) as i32 - (data[5] & 0x01 != 0) as i32;
                dev.report_abs(AbsoluteAxis::Hat0X, hat_x);
                dev.report_abs(AbsoluteAxis::Hat0Y, hat_y);
            }

            // Sticks and triggers
            if !xpad.mapping.contains(MapFlags::STICKS_TO_NULL) {
                dev.report_abs(AbsoluteAxis::X, i16::from_le_bytes([data[10], data[11]]).into());
                dev.report_abs(AbsoluteAxis::Y, (!i16::from_le_bytes([data[12], data[13]])).into());
                dev.report_abs(AbsoluteAxis::Rx, i16::from_le_bytes([data[14], data[15]]).into());
                dev.report_abs(AbsoluteAxis::Ry, (!i16::from_le_bytes([data[16], data[17]])).into());
            }

            do_sync = true;
        },
        0x21 => {
            // GHL guitar processing
            let dpad_value = data[6] & 0x0F;
            let (x, y) = DPAD_MAPPING[dpad_value.min(8) as usize];
            dev.report_abs(AbsoluteAxis::Hat0X, x);
            dev.report_abs(AbsoluteAxis::Hat0Y, y);
            do_sync = true;
        },
        _ => (),
    }

    if do_sync {
        dev.synchronize();
    }
}

// URB completion handler
fn xpad_irq_in(urb: &Urb, xpad: Arc<UsbXpad>) -> Result<(), UsbError> {
    match urb.status() {
        UsbStatus::Success => (),
        UsbStatus::Disconnected | UsbStatus::Cancelled => return Ok(()),
        err => {
            log::warn!("URB error: {:?}", err);
            return Err(err.into());
        }
    }

    let data = urb.buffer();
    log::debug!("Received packet: {:02X?}", data);

    match xpad.xtype {
        XType::Xbox360 => xpad360_process_packet(&xpad.dev, data),
        XType::Xbox360W => xpad360w_process_packet(&xpad, data),
        XType::XboxOne => xpadone_process_packet(&xpad, data),
        _ => xpad_process_packet(&xpad, data),
    }

    // Resubmit URB
    urb.submit()?;
    Ok(())
}

// Initialization sequence handling
fn xpad_prepare_next_init_packet(xpad: &UsbXpad) -> Option<Vec<u8>> {
    let mut seq = xpad.init_seq.lock().unwrap();
    while *seq < XBOXONE_INIT_PACKETS.len() {
        let packet = &XBOXONE_INIT_PACKETS[*seq];
        *seq += 1;

        if (packet.vendor == 0 || packet.vendor == xpad.device.vendor_id()) &&
           (packet.product == 0 || packet.product == xpad.device.product_id()) {
            let mut data = packet.data.to_vec();
            data[2] = xpad.odata_serial.fetch_add(1, Ordering::SeqCst) as u8;
            return Some(data);
        }
    }
    None
}

// Output packet handling
fn xpad_try_sending_next_out_packet(xpad: &UsbXpad) -> Result<(), UsbError> {
    let mut odata = xpad.odata.lock().unwrap();
    
    if let Some(init_data) = xpad_prepare_next_init_packet(xpad) {
        *odata = init_data;
        xpad.irq_out.submit(&odata)?;
        return Ok(());
    }

    // Regular output packet handling would go here
    Ok(())
}

// Force feedback implementation
fn xpad_play_effect(xpad: &UsbXpad, strong: u16, weak: u16) -> Result<(), UsbError> {
    let mut packet = Vec::with_capacity(13);
    
    match xpad.xtype {
        XType::XboxOne => {
            packet.extend_from_slice(&[
                GIP_CMD_RUMBLE,
                0x00,
                xpad.odata_serial.fetch_add(1, Ordering::SeqCst) as u8,
                GIP_PL_LEN(9),
                0x00,
                GIP_MOTOR_ALL,
                0x00,
                0x00,
                (strong / 512) as u8,
                (weak / 512) as u8,
                0xFF,
                0x00,
                0xFF,
            ]);
        },
        // Other controller types...
        _ => return Err(UsbError::NotSupported),
    }

    xpad.send_output_packet(&packet)
}

// LED control
struct XpadLed {
    xpad: Arc<UsbXpad>,
    // LED state would be maintained here
}

impl LedDevice for XpadLed {
    fn set_state(&mut self, state: LedState) -> Result<(), DeviceError> {
        let packet = match state {
            LedState::Pattern(pattern) => create_led_packet(pattern),
            // Other states...
        };
        self.xpad.send_output_packet(&packet)
    }
}
