use core::ffi::c_void;

use windows_sys::Win32::{
    Foundation::HANDLE,
    NetworkManagement::{
        IpHelper::IP_ADDRESS_PREFIX,
        WindowsFilteringPlatform::{
            FWPS_METADATA_FIELD_PROCESS_ID, FWPS_METADATA_FIELD_PROCESS_PATH, FWP_BYTE_BLOB,
            FWP_DIRECTION,
        },
    },
    Networking::WinSock::SCOPE_ID,
};

#[repr(C)]
pub(crate) struct FwpsIncomingMetadataValues {
    /// Bitmask representing which values are set.
    current_metadata_values: u32,
    /// Internal flags;
    flags: u32,
    /// Reserved for system use.
    reserved: u64,
    /// Discard module and reason.
    discard_metadata: FwpsDiscardMetadata0,
    /// Flow Handle.
    flow_handle: u64,
    /// IP Header size.
    ip_header_size: u32,
    /// Transport Header size
    transport_header_size: u32,
    /// Process Path.
    process_path: *const FWP_BYTE_BLOB,
    /// Token used for authorization.
    token: u64,
    /// Process Id.
    process_id: u64,
    /// Source and Destination interface indices for discard indications.
    source_interface_index: u32,
    destination_interface_index: u32,
    /// Compartment Id for injection APIs.
    compartment_id: u32,
    /// Fragment data for inbound fragments.
    fragment_metadata: FwpsInboundFragmentMetadata0,
    /// Path MTU for outbound packets (to enable calculation of fragments).
    path_mtu: u32,
    /// Completion handle (required in order to be able to pend at this layer).
    completion_handle: HANDLE,
    /// Endpoint handle for use in outbound transport layer injection.
    transport_endpoint_handle: u64,
    /// Remote scope id for use in outbound transport layer injection.
    remote_scope_id: SCOPE_ID,
    /// Socket control data (and length) for use in outbound transport layer injection.
    control_data: *const c_void,
    control_data_length: u32,
    /// Direction for the current packet. Only specified for ALE re-authorization.
    packet_direction: FWP_DIRECTION,
    /// Raw IP header (and length) if the packet is sent with IP header from a RAW socket.
    header_include_header: *mut c_void,
    header_include_header_length: u32,
    destination_prefix: IP_ADDRESS_PREFIX,
    frame_length: u16,
    parent_endpoint_handle: u64,
    icmp_id_and_sequence: u32,
    /// PID of the process that will be accepting the redirected connection
    local_redirect_target_pid: u64,
    /// original destination of a redirected connection
    original_destination: *mut c_void,
    redirect_records: HANDLE,
    /// Bitmask representing which L2 values are set.
    current_l2_metadata_values: u32,
    /// L2 layer Flags;
    l2_flags: u32,
    ethernet_mac_header_size: u32,
    wifi_operation_mode: u32,
    padding0: u32,
    padding1: u16,
    padding2: u32,
    v_switch_packet_context: HANDLE,
    sub_process_tag: *mut c_void,
    // Reserved for system use.
    reserved1: u64,
}

impl FwpsIncomingMetadataValues {
    pub(crate) fn has_field(&self, field: u32) -> bool {
        self.current_metadata_values & field > 0
    }

    pub(crate) fn get_process_id(&self) -> Option<u64> {
        if self.has_field(FWPS_METADATA_FIELD_PROCESS_ID) {
            return Some(self.process_id);
        }

        return None;
    }

    pub(crate) unsafe fn get_process_path(&self) -> Option<&[u8]> {
        if self.has_field(FWPS_METADATA_FIELD_PROCESS_PATH) {
            return Some(core::slice::from_raw_parts(
                (*self.process_path).data,
                (*self.process_path).size as usize,
            ));
        }

        return None;
    }
}

#[allow(dead_code)]
#[repr(C)]
enum FwpsDiscardModule0 {
    FwpsDiscardModuleNetwork = 0,
    FwpsDiscardModuleTransport = 1,
    FwpsDiscardModuleGeneral = 2,
    FwpsDiscardModuleMax = 3,
}

#[repr(C)]
struct FwpsDiscardMetadata0 {
    discard_module: FwpsDiscardModule0,
    discard_reason: u32,
    filter_id: u64,
}

#[repr(C)]
struct FwpsInboundFragmentMetadata0 {
    fragment_identification: u32,
    fragment_offset: u16,
    fragment_length: u32,
}