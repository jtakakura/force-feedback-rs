// exports from `MacTypes.h`
pub type UInt32 = ::std::os::raw::c_uint;
pub type SInt32 = ::std::os::raw::c_int;
pub type HRESULT = ::std::os::raw::c_long;

// Version Release Stage Codes
pub const developStage: u32 = 0x20;
pub const alphaStage: u32 = 0x40;
pub const betaStage: u32 = 0x60;
pub const finalStage: u32 = 0x80;

// Common HRESULT Values
pub const S_OK: HRESULT = 0x00000000;
pub const S_FALSE: HRESULT = 0x00000001;
pub const E_NOTIMPL: HRESULT = 0x80004001;
pub const E_NOINTERFACE: HRESULT = 0x80004002;
pub const E_POINTER: HRESULT = 0x80004003;
pub const E_ABORT: HRESULT = 0x80004004;
pub const E_FAIL: HRESULT = 0x80004005;
pub const E_UNEXPECTED: HRESULT = 0x8000FFFF;
pub const E_ACCESSDENIED: HRESULT = 0x80070005;
pub const E_HANDLE: HRESULT = 0x80070006;
pub const E_OUTOFMEMORY: HRESULT = 0x8007000E;
pub const E_INVALIDARG: HRESULT = 0x80070057;

pub const REGDB_E_CLASSNOTREG: HRESULT = 0x80040154;
