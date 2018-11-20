#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate core_foundation_sys;
extern crate io_kit_sys;
extern crate mach;

pub mod base;
pub mod constants;

use core_foundation_sys::uuid::CFUUIDRef;

use io_kit_sys::types::{io_service_t, IOByteCount};

use base::*;
use constants::*;

// exports from <ForceFeedback/ForceFeedback.h>

// Force Feedback API Version
pub const kFFAPIMajorRev: u32 = 1;
pub const kFFAPIMinorAndBugRev: u32 = 0;
pub const kFFAPIStage: u32 = finalStage;
pub const kFFAPINonRelRev: u32 = 0;

// Effect definition structures
// Contains type-specific information for the CONSTANTFORCE effect.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCONSTANTFORCE {
    pub lMagnitude: LONG,
}
pub type PFFCONSTANTFORCE = *mut FFCONSTANTFORCE;

// Contains type-specific information for the RAMPFORCE effect.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFRAMPFORCE {
    pub lStart: LONG,
    pub lEnd: LONG,
}
pub type PFFRAMPFORCE = *mut FFRAMPFORCE;

// Contains type-specific information for the following effects.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFPERIODIC {
    pub dwMagnitude: DWORD,
    pub lOffset: LONG,
    pub dwPhase: DWORD,
    pub dwPeriod: DWORD,
}
pub type PFFPERIODIC = *mut FFPERIODIC;

// Contains type-specific information for the following effects.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCONDITION {
    pub lOffset: LONG,
    pub lPositiveCoefficient: LONG,
    pub lNegativeCoefficient: LONG,
    pub dwPositiveSaturation: DWORD,
    pub dwNegativeSaturation: DWORD,
    pub lDeadBand: LONG,
}
pub type PFFCONDITION = *mut FFCONDITION;

// Contains type-specific information for the CUSTOMFORCE effect.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCUSTOMFORCE {
    pub cChannels: DWORD,
    pub dwSamplePeriod: DWORD,
    pub cSamples: DWORD,
    pub rglForceData: LPLONG,
}
pub type PFFCUSTOMFORCE = *mut FFCUSTOMFORCE;

// Used by the FFEFFECT structure to specify the optional envelope parameters for an effect.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFENVELOPE {
    pub dwSize: DWORD,
    pub dwAttackLevel: DWORD,
    pub dwAttackTime: DWORD,
    pub dwFadeLevel: DWORD,
    pub dwFadeTime: DWORD,
}
pub type PFFENVELOPE = *mut FFENVELOPE;

// UsUsed by the FFDeviceCreateEffect method to initialize a new effect object. It is also used by the FFEffectSetParameters and FFEffectGetParameters functions.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFEFFECT {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub dwDuration: DWORD,
    pub dwSamplePeriod: DWORD,
    pub dwGain: DWORD,
    pub dwTriggerButton: DWORD,
    pub dwTriggerRepeatInterval: DWORD,
    pub cAxes: DWORD,
    pub rgdwAxes: LPDWORD,
    pub rglDirection: LPLONG,
    pub lpEnvelope: PFFENVELOPE,
    pub cbTypeSpecificParams: DWORD,
    pub lpvTypeSpecificParams: *mut ::std::os::raw::c_void,
    pub dwStartDelay: DWORD,
}
pub type PFFEFFECT = *mut FFEFFECT;

// The FFEFFESCAPE structure passes hardware-specific data directly to the Force Feedback plugIn.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFEFFESCAPE {
    pub dwSize: DWORD,
    pub dwCommand: DWORD,
    pub lpvInBuffer: *mut ::std::os::raw::c_void,
    pub cbInBuffer: DWORD,
    pub lpvOutBuffer: *mut ::std::os::raw::c_void,
    pub cbOutBuffer: DWORD,
}
pub type PFFEFFESCAPE = *mut FFEFFESCAPE;

// Used by the FFDeviceGetForceFeedbackCapabilities method to retrieve device force-feedback capabilities.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCAPABILITIES {
    pub ffSpecVer: NumVersion,
    pub supportedEffects: UInt32,
    pub emulatedEffects: UInt32,
    pub subType: UInt32,
    pub numFfAxes: UInt32,
    pub ffAxes: [UInt8; 32usize],
    pub storageCapacity: UInt32,
    pub playbackCapacity: UInt32,
    pub firmwareVer: NumVersion,
    pub hardwareVer: NumVersion,
    pub driverVer: NumVersion,
}
pub type PFFCAPABILITIES = *mut FFCAPABILITIES;

// Object reference pointers
// FFDeviceObjectReference and FFEffectObjectReference are opaque handles
// to objects created and maintained by the FF API.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FFDHIDDEN {}
pub type FFDeviceObjectReference = *mut __FFDHIDDEN;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FFEHIDDEN {}
pub type FFEffectObjectReference = *mut __FFEHIDDEN;

extern "C" {
    // FF (general) function prototypes
    // Creates a new API device object from an OS object in preparation to use the device for force feedback.
    pub fn FFCreateDevice(
        hidDevice: io_service_t,
        pDeviceReference: *mut FFDeviceObjectReference,
    ) -> HRESULT;

    // Disposes of an API device object created with FFCreateDevice.
    pub fn FFReleaseDevice(deviceReference: FFDeviceObjectReference) -> HRESULT;

    // Used to determine if a particular device provided by HID Manager is a force feedback device.
    pub fn FFIsForceFeedback(hidDevice: io_service_t) -> HRESULT;

    // FFDevice (device related) function prototypes
    // Creates and initializes an instance of an effect identified by the effect UUID on the device.
    pub fn FFDeviceCreateEffect(
        deviceReference: FFDeviceObjectReference,
        uuidRef: CFUUIDRef,
        pEffectDefinition: *mut FFEFFECT,
        pEffectReference: *mut FFEffectObjectReference,
    ) -> HRESULT;

    // Disposes of an API effect object created with FFDeviceCreateEffect.
    pub fn FFDeviceReleaseEffect(
        deviceReference: FFDeviceObjectReference,
        effectReference: FFEffectObjectReference,
    ) -> HRESULT;

    // Sends a hardware-specific command to the device.
    pub fn FFDeviceEscape(
        deviceReference: FFDeviceObjectReference,
        pFFEffectEscape: *mut FFEFFESCAPE,
    ) -> HRESULT;

    // Retrieves the state of the device's force feedback system.
    pub fn FFDeviceGetForceFeedbackState(
        deviceReference: FFDeviceObjectReference,
        pFFState: *mut FFState,
    ) -> HRESULT;

    // Sends a command to the device's force feedback system.
    pub fn FFDeviceSendForceFeedbackCommand(
        deviceReference: FFDeviceObjectReference,
        flags: FFCommandFlag,
    ) -> HRESULT;

    // Retrieves the device's force feedback capabilities.
    pub fn FFDeviceSetForceFeedbackProperty(
        deviceReference: FFDeviceObjectReference,
        property: FFProperty,
        pValue: *mut ::std::os::raw::c_void,
    ) -> HRESULT;

    // Gets properties that define the device behavior.
    pub fn FFDeviceGetForceFeedbackProperty(
        deviceReference: FFDeviceObjectReference,
        property: FFProperty,
        pValue: *mut ::std::os::raw::c_void,
        valueSize: IOByteCount,
    ) -> HRESULT;

    // Function is unimplemented in version 1.0 of this API
    pub fn FFDeviceSetCooperativeLevel(
        deviceReference: FFDeviceObjectReference,
        taskIdentifier: *mut ::std::os::raw::c_void,
        flags: FFCooperativeLevelFlag,
    ) -> HRESULT;

    // Retrieves the device's force feedback capabilities.
    pub fn FFDeviceGetForceFeedbackCapabilities(
        deviceReference: FFDeviceObjectReference,
        pFFCapabilities: *mut FFCAPABILITIES,
    ) -> HRESULT;

    // FFEffect (effect related) function prototypes
    // Places the effect on the device. If the effect is already on the device, the existing effect is updated to match the values set by the FFEffectSetParameters method.
    pub fn FFEffectDownload(effectReference: FFEffectObjectReference) -> HRESULT;

    // Sends a hardware-specific command to the driver.
    pub fn FFEffectEscape(
        effectReference: FFEffectObjectReference,
        pFFEffectEscape: *mut FFEFFESCAPE,
    ) -> HRESULT;

    // Sends a hardware-specific command to the driver.
    pub fn FFEffectGetEffectStatus(
        effectReference: FFEffectObjectReference,
        pFlags: *mut FFEffectStatusFlag,
    ) -> HRESULT;

    // Retrieves information about an effect.
    pub fn FFEffectGetParameters(
        effectReference: FFEffectObjectReference,
        pFFEffect: *mut FFEFFECT,
        flags: FFEffectParameterFlag,
    ) -> HRESULT;

    // Sets the characteristics of an effect.
    pub fn FFEffectSetParameters(
        effectReference: FFEffectObjectReference,
        pFFEffect: *mut FFEFFECT,
        flags: FFEffectParameterFlag,
    ) -> HRESULT;

    // Begins playing an effect. If the effect is already playing, it is restarted from the beginning. If the effect has not been downloaded or has been modified since its last download, it is downloaded before being started.
    pub fn FFEffectStart(
        effectReference: FFEffectObjectReference,
        iterations: UInt32,
        flags: FFEffectStartFlag,
    ) -> HRESULT;

    // Stops playing an effect.
    pub fn FFEffectStop(effectReference: FFEffectObjectReference) -> HRESULT;

    // Removes the effect from the device. If the effect is playing, it is automatically stopped before it is unloaded.
    pub fn FFEffectUnload(effectReference: FFEffectObjectReference) -> HRESULT;
}
