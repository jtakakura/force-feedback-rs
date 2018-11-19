// exports from <ForceFeedback/ForceFeedbackConstants.h>

use core_foundation_sys::uuid::CFUUIDRef;

use base::*;

// General defines
pub type DWORD = UInt32;
pub type LPDWORD = *mut DWORD;
pub type LONG = SInt32;
pub type LPLONG = *mut LONG;

pub const FF_INFINITE: ::std::os::raw::c_uint = 0xFFFFFFFF;
pub const FF_DEGREES: ::std::os::raw::c_uint = 100;
pub const FF_FFNOMINALMAX: ::std::os::raw::c_uint = 10000;
pub const FF_SECONDS: ::std::os::raw::c_uint = 1000000;

// Effect definition constants
// FFEFF_OBJECTOFFSETS
pub const FFEFF_OBJECTOFFSETS: ::std::os::raw::c_uint = 0x00000002;

// FFCoordinateSystemFlag
pub const FFEFF_CARTESIAN: FFCoordinateSystemFlag = 0x00000010;
pub const FFEFF_POLAR: FFCoordinateSystemFlag = 0x00000020;
pub const FFEFF_SPHERICAL: FFCoordinateSystemFlag = 0x00000040;
pub type FFCoordinateSystemFlag = UInt32;

// FFEffectParameterFlag
pub const FFEP_DURATION: FFEffectParameterFlag = 0x00000001;
pub const FFEP_SAMPLEPERIOD: FFEffectParameterFlag = 0x00000002;
pub const FFEP_GAIN: FFEffectParameterFlag = 0x00000004;
pub const FFEP_TRIGGERBUTTON: FFEffectParameterFlag = 0x00000008;
pub const FFEP_TRIGGERREPEATINTERVAL: FFEffectParameterFlag = 0x00000010;
pub const FFEP_AXES: FFEffectParameterFlag = 0x00000020;
pub const FFEP_DIRECTION: FFEffectParameterFlag = 0x00000040;
pub const FFEP_ENVELOPE: FFEffectParameterFlag = 0x00000080;
pub const FFEP_TYPESPECIFICPARAMS: FFEffectParameterFlag = 0x00000100;
pub const FFEP_STARTDELAY: FFEffectParameterFlag = 0x00000200;
pub const FFEP_ALLPARAMS: FFEffectParameterFlag = 0x000003FF;
pub const FFEP_START: FFEffectParameterFlag = 0x20000000;
pub const FFEP_NORESTART: FFEffectParameterFlag = 0x40000000;
pub const FFEP_NODOWNLOAD: FFEffectParameterFlag = 0x80000000;
pub const FFEB_NOTRIGGER: FFEffectParameterFlag = 0xFFFFFFFF;
pub type FFEffectParameterFlag = UInt32;

// FFEffectStartFlag
pub const FFES_SOLO: FFEffectStartFlag = 0x00000001;
pub const FFES_NODOWNLOAD: FFEffectStartFlag = 0x80000000;
pub type FFEffectStartFlag = UInt32;

// FFEffectStatusFlag
pub const FFEGES_NOTPLAYING: FFEffectStatusFlag = 0x00000000;
pub const FFEGES_PLAYING: FFEffectStatusFlag = 0x00000001;
pub const FFEGES_EMATED: FFEffectStatusFlag = 0x00000002;
pub type FFEffectStatusFlag = UInt32;

// FFCommandFlag
pub const FFSFFC_RESET: FFCommandFlag = 0x00000001;
pub const FFSFFC_STOPALL: FFCommandFlag = 0x00000002;
pub const FFSFFC_PAUSE: FFCommandFlag = 0x00000004;
pub const FFSFFC_CONTINUE: FFCommandFlag = 0x00000008;
pub const FFSFFC_SETACTUATORSON: FFCommandFlag = 0x00000010;
pub const FFSFFC_SETACTUATORSOFF: FFCommandFlag = 0x00000020;
pub type FFCommandFlag = UInt32;

// FFState
pub const FFGFFS_EMPTY: FFState = 0x00000001;
pub const FFGFFS_STOPPED: FFState = 0x00000002;
pub const FFGFFS_PAUSED: FFState = 0x00000004;
pub const FFGFFS_ACTUATORSON: FFState = 0x00000010;
pub const FFGFFS_ACTUATORSOFF: FFState = 0x00000020;
pub const FFGFFS_POWERON: FFState = 0x00000040;
pub const FFGFFS_POWEROFF: FFState = 0x00000080;
pub const FFGFFS_SAFETYSWITCHON: FFState = 0x00000100;
pub const FFGFFS_SAFETYSWITCHOFF: FFState = 0x00000200;
pub const FFGFFS_USERFFSWITCHON: FFState = 0x00000400;
pub const FFGFFS_USERFFSWITCHOFF: FFState = 0x00000800;
pub const FFGFFS_DEVICELOST: FFState = 0x80000000;
pub type FFState = UInt32;

// FFJOFS_i
pub const FFJOFS_X: u32 = 0;
pub const FFJOFS_Y: u32 = 4;
pub const FFJOFS_Z: u32 = 8;
pub const FFJOFS_RX: u32 = 12;
pub const FFJOFS_RY: u32 = 16;
pub const FFJOFS_RZ: u32 = 20;
pub const FFJOFS_BUTTON0: u32 = 48;
pub const FFJOFS_BUTTON1: u32 = FFJOFS_BUTTON0 + 1;
pub const FFJOFS_BUTTON2: u32 = FFJOFS_BUTTON0 + 2;
pub const FFJOFS_BUTTON3: u32 = FFJOFS_BUTTON0 + 3;
pub const FFJOFS_BUTTON4: u32 = FFJOFS_BUTTON0 + 4;
pub const FFJOFS_BUTTON5: u32 = FFJOFS_BUTTON0 + 5;
pub const FFJOFS_BUTTON6: u32 = FFJOFS_BUTTON0 + 6;
pub const FFJOFS_BUTTON7: u32 = FFJOFS_BUTTON0 + 7;
pub const FFJOFS_BUTTON8: u32 = FFJOFS_BUTTON0 + 8;
pub const FFJOFS_BUTTON9: u32 = FFJOFS_BUTTON0 + 9;
pub const FFJOFS_BUTTON10: u32 = FFJOFS_BUTTON0 + 10;
pub const FFJOFS_BUTTON11: u32 = FFJOFS_BUTTON0 + 11;
pub const FFJOFS_BUTTON12: u32 = FFJOFS_BUTTON0 + 12;
pub const FFJOFS_BUTTON13: u32 = FFJOFS_BUTTON0 + 13;
pub const FFJOFS_BUTTON14: u32 = FFJOFS_BUTTON0 + 14;
pub const FFJOFS_BUTTON15: u32 = FFJOFS_BUTTON0 + 15;
pub const FFJOFS_BUTTON16: u32 = FFJOFS_BUTTON0 + 16;
pub const FFJOFS_BUTTON17: u32 = FFJOFS_BUTTON0 + 17;
pub const FFJOFS_BUTTON18: u32 = FFJOFS_BUTTON0 + 18;
pub const FFJOFS_BUTTON19: u32 = FFJOFS_BUTTON0 + 19;
pub const FFJOFS_BUTTON20: u32 = FFJOFS_BUTTON0 + 20;
pub const FFJOFS_BUTTON21: u32 = FFJOFS_BUTTON0 + 21;
pub const FFJOFS_BUTTON22: u32 = FFJOFS_BUTTON0 + 22;
pub const FFJOFS_BUTTON23: u32 = FFJOFS_BUTTON0 + 23;
pub const FFJOFS_BUTTON24: u32 = FFJOFS_BUTTON0 + 24;
pub const FFJOFS_BUTTON25: u32 = FFJOFS_BUTTON0 + 25;
pub const FFJOFS_BUTTON26: u32 = FFJOFS_BUTTON0 + 26;
pub const FFJOFS_BUTTON27: u32 = FFJOFS_BUTTON0 + 27;
pub const FFJOFS_BUTTON28: u32 = FFJOFS_BUTTON0 + 28;
pub const FFJOFS_BUTTON29: u32 = FFJOFS_BUTTON0 + 29;
pub const FFJOFS_BUTTON30: u32 = FFJOFS_BUTTON0 + 30;
pub const FFJOFS_BUTTON31: u32 = FFJOFS_BUTTON0 + 31;

// FFProperty
pub const FFPROP_FFGAIN: FFProperty = 1;
pub const FFPROP_AUTOCENTER: FFProperty = 3;
pub type FFProperty = UInt32;

// FFCooperativeLevelFlag
pub const FFSCL_EXCLUSIVE: FFCooperativeLevelFlag = 0x00000001;
pub const FFSCL_NONEXCLUSIVE: FFCooperativeLevelFlag = 0x00000002;
pub const FFSCL_FOREGROUND: FFCooperativeLevelFlag = 0x00000004;
pub const FFSCL_BACKGROUND: FFCooperativeLevelFlag = 0x00000008;
pub type FFCooperativeLevelFlag = UInt32;

// FFCapabilitiesEffectType
pub const FFCAP_ET_CONSTANTFORCE: FFCapabilitiesEffectType = 0x00000001;
pub const FFCAP_ET_RAMPFORCE: FFCapabilitiesEffectType = 0x00000002;
pub const FFCAP_ET_SQUARE: FFCapabilitiesEffectType = 0x00000004;
pub const FFCAP_ET_SINE: FFCapabilitiesEffectType = 0x00000008;
pub const FFCAP_ET_TRIANGLE: FFCapabilitiesEffectType = 0x00000010;
pub const FFCAP_ET_SAWTOOTHUP: FFCapabilitiesEffectType = 0x00000020;
pub const FFCAP_ET_SAWTOOTHDOWN: FFCapabilitiesEffectType = 0x00000040;
pub const FFCAP_ET_SPRING: FFCapabilitiesEffectType = 0x00000080;
pub const FFCAP_ET_DAMPER: FFCapabilitiesEffectType = 0x00000100;
pub const FFCAP_ET_INERTIA: FFCapabilitiesEffectType = 0x00000200;
pub const FFCAP_ET_FRICTION: FFCapabilitiesEffectType = 0x00000400;
pub const FFCAP_ET_CUSTOMFORCE: FFCapabilitiesEffectType = 0x00000800;
pub type FFCapabilitiesEffectType = UInt32;

// FFCapabilitiesEffectSubType
pub const FFCAP_ST_KINESTHETIC: FFCapabilitiesEffectSubType = 1;
pub const FFCAP_ST_VIBRATION: FFCapabilitiesEffectSubType = 2;
pub type FFCapabilitiesEffectSubType = UInt32;

// Error return values
pub const FF_OK: HRESULT = S_OK;
pub const FF_FALSE: HRESULT = S_FALSE;

pub const FF_DOWNLOADSKIPPED: HRESULT = 0x00000003;
pub const FF_EFFECTRESTARTED: HRESULT = 0x00000004;
pub const FF_TRUNCATED: HRESULT = 0x00000008;
pub const FF_TRUNCATEDANDRESTARTED: HRESULT = 0x0000000;
pub const FFERR_DEVICENOTREG: HRESULT = REGDB_E_CLASSNOTREG;
pub const FFERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
pub const FFERR_NOINTERFACE: HRESULT = E_NOINTERFACE;
pub const FFERR_GENERIC: HRESULT = E_FAIL;
pub const FFERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const FFERR_UNSUPPORTED: HRESULT = E_NOTIMPL;
pub const E_PENDING: HRESULT = 0x8000000A;
pub const FFERR_DEVICEFULL: HRESULT = 0x80040201;
pub const FFERR_MOREDATA: HRESULT = 0x80040202;
pub const FFERR_NOTDOWNLOADED: HRESULT = 0x80040203;
pub const FFERR_HASEFFECTS: HRESULT = 0x80040204;
pub const FFERR_INCOMPLETEEFFECT: HRESULT = 0x80040206;
pub const FFERR_EFFECTPLAYING: HRESULT = 0x80040208;
pub const FFERR_UNPLUGGED: HRESULT = 0x80040209;
pub const FFERR_INVALIDDOWNLOADID: HRESULT = 0x80040300;
pub const FFERR_DEVICEPAUSED: HRESULT = 0x80040301;
pub const FFERR_INTERNAL: HRESULT = 0x80040302;
pub const FFERR_EFFECTTYPEMISMATCH: HRESULT = 0x80040303;
pub const FFERR_UNSUPPORTEDAXIS: HRESULT = 0x80040304;
pub const FFERR_NOTINITIALIZED: HRESULT = 0x80040305;
pub const FFERR_EFFECTTYPENOTSUPPORTED: HRESULT = 0x80040306;
pub const FFERR_DEVICERELEASED: HRESULT = 0x80040307;

extern "C" {
    // Effect type UUIDs
    // E559C460-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_ConstantForce_ID: CFUUIDRef;
    // E559C461-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_RampForce_ID: CFUUIDRef;
    // E559C462-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Square_ID: CFUUIDRef;
    // E559C463-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Sine_ID: CFUUIDRef;
    // E559C464-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Triangle_ID: CFUUIDRef;
    // E559C465-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_SawtoothUp_ID: CFUUIDRef;
    // E559C466-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_SawtoothDown_ID: CFUUIDRef;
    // E559C467-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Spring_ID: CFUUIDRef;
    // E559C468-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Damper_ID: CFUUIDRef;
    // E559C469-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Inertia_ID: CFUUIDRef;
    // E559C46A-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_Friction_ID: CFUUIDRef;
    // E559C46B-C5CD-11D6-8A1C-00039353BD00
    pub static kFFEffectType_CustomForce_ID: CFUUIDRef;
}
