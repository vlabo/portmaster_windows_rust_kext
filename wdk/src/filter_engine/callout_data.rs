use crate::utils::check_ntstatus;

use super::{
    classify::ClassifyOut,
    connect_request::FwpsConnectRequest0,
    ffi::{self, FwpsPendOperation0},
    layer::{Layer, Value},
    metadata::FwpsIncomingMetadataValues,
    packet::PacketList,
    FilterEngine,
};
use alloc::string::{String, ToString};
use core::ffi::c_void;
use windows_sys::Win32::{
    Foundation::HANDLE,
    NetworkManagement::WindowsFilteringPlatform::FWP_CONDITION_FLAG_IS_REAUTHORIZE,
    Networking::WinSock::SCOPE_ID,
};

pub enum ClassifyPromise {
    Initial(HANDLE, Option<PacketList>),
    Reauthorization(usize, Option<PacketList>),
}

impl ClassifyPromise {
    pub fn complete(self, filter_engine: &FilterEngine) -> Result<Option<PacketList>, String> {
        unsafe {
            match self {
                ClassifyPromise::Initial(context, packet_list) => {
                    ffi::FwpsCompleteOperation0(context, core::ptr::null_mut());
                    return Ok(packet_list);
                }
                ClassifyPromise::Reauthorization(callout_index, packet_list) => {
                    filter_engine.reset_callout_filter(callout_index)?;
                    return Ok(packet_list);
                }
            }
        }
    }
}

pub struct CalloutData<'a> {
    pub layer: Layer,
    pub(crate) callout_index: usize,
    pub(crate) values: &'a [Value],
    pub(crate) metadata: *const FwpsIncomingMetadataValues,
    pub(crate) classify_out: *mut ClassifyOut,
    pub(crate) classify_context: *mut c_void,
    pub(crate) filter_id: u64,
    pub(crate) layer_data: *mut c_void,
}

impl<'a> CalloutData<'a> {
    pub fn get_value_u8(&'a self, index: usize) -> u8 {
        unsafe {
            return self.values[index].value.uint8;
        };
    }

    pub fn get_value_u16(&'a self, index: usize) -> u16 {
        unsafe {
            return self.values[index].value.uint16;
        };
    }

    pub fn get_value_u32(&'a self, index: usize) -> u32 {
        unsafe {
            return self.values[index].value.uint32;
        };
    }

    pub fn get_process_id(&self) -> Option<u64> {
        unsafe { (*self.metadata).get_process_id() }
    }

    pub fn get_process_path(&self) -> Option<String> {
        unsafe {
            return (*self.metadata).get_process_path();
        }
    }

    pub fn get_transport_endpoint_handle(&self) -> Option<u64> {
        unsafe {
            return (*self.metadata).get_transport_endpoint_handle();
        }
    }

    pub fn get_remote_scope_id(&self) -> Option<SCOPE_ID> {
        unsafe {
            return (*self.metadata).get_remote_scope_id();
        }
    }

    pub fn get_control_data(&self) -> Option<&[u8]> {
        unsafe {
            return (*self.metadata).get_control_data();
        }
    }

    pub fn get_layer_data(&self) -> *mut c_void {
        return self.layer_data;
    }

    pub fn pend_operation(
        &mut self,
        packet_list: Option<PacketList>,
    ) -> Result<ClassifyPromise, String> {
        unsafe {
            let mut completion_context = 0;
            if let Some(completion_handle) = (*self.metadata).get_completeion_handle() {
                let status = FwpsPendOperation0(completion_handle, &mut completion_context);
                check_ntstatus(status)?;

                return Ok(ClassifyPromise::Initial(completion_context, packet_list));
            }

            Err("callout not supported".to_string())
        }
    }

    pub fn pend_filter_rest(&mut self, packet_list: Option<PacketList>) -> ClassifyPromise {
        return ClassifyPromise::Reauthorization(self.callout_index, packet_list);
    }

    pub fn action_permit(&mut self) {
        unsafe {
            (*self.classify_out).action_permit();
        }
    }

    pub fn action_continue(&mut self) {
        unsafe {
            (*self.classify_out).action_continue();
        }
    }

    pub fn action_block(&mut self) {
        unsafe {
            (*self.classify_out).action_block();
            (*self.classify_out).clear_write_flag();
        }
    }

    pub fn block_and_absorb(&mut self) {
        unsafe {
            (*self.classify_out).action_block();
            (*self.classify_out).set_absorb();
            (*self.classify_out).clear_write_flag();
        }
    }

    pub fn is_reauthorize(&self, flags_index: usize) -> bool {
        self.get_value_u32(flags_index) & FWP_CONDITION_FLAG_IS_REAUTHORIZE > 0
    }
}