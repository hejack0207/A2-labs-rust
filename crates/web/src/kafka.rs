#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sim_msg_header {
    pub simulationId: *mut ::std::os::raw::c_char,
    pub sceneObjectId: *mut ::std::os::raw::c_char,
    pub simulationTime: ::std::os::raw::c_long,
    pub timestamp: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sim_status_reply {
    pub header: sim_msg_header,
    pub status: i8,
    pub msg: *mut ::std::os::raw::c_char,
    pub containerId: *mut ::std::os::raw::c_char,
    pub containerIp: *mut ::std::os::raw::c_char,
    pub port: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sim_status_data {
    pub header: sim_msg_header,
    pub sceneObjectId: *mut ::std::os::raw::c_char,
    pub simulationTime: ::std::os::raw::c_long,
    pub rrcLinkStatus: u8,
    pub dlSpeed: u32,
    pub ulSpeed: u32,
    pub timeDelay: u32,
    pub plmn: *mut ::std::os::raw::c_char,
    pub ueIp: *mut ::std::os::raw::c_char,
    pub ueId: *mut ::std::os::raw::c_char,
    pub optType: i8,
}

pub const SIM_LOG_LEVEL_FATAL: SIM_LOG_LEVEL = 1;
pub const SIM_LOG_LEVEL_ERROR: SIM_LOG_LEVEL = 2;
pub const SIM_LOG_LEVEL_WARN: SIM_LOG_LEVEL = 3;
pub const SIM_LOG_LEVEL_INFO: SIM_LOG_LEVEL = 4;
pub type SIM_LOG_LEVEL = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sim_running_log {
    pub header: sim_msg_header,
    pub sceneObjectId: *mut ::std::os::raw::c_char,
    pub simulationTime: ::std::os::raw::c_long,
    pub logType: SIM_LOG_LEVEL,
    pub logDomain: *mut ::std::os::raw::c_char,
    pub logData: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sim_model_event {
    pub header: sim_msg_header,
    pub sceneObjectId: *mut ::std::os::raw::c_char,
    pub simulationTime: ::std::os::raw::c_long,
    pub eventType: *mut ::std::os::raw::c_char,
    pub returnMean: *mut ::std::os::raw::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn send_sim_status_reply(p_sim_status_reply: *mut sim_status_reply,) -> ::std::os::raw::c_int{
    return 0;
}

#[no_mangle]
pub extern "C" fn send_sim_status_data(p_sim_status_data: *mut sim_status_data) -> ::std::os::raw::c_int{
    return 0;
}

#[no_mangle]
pub extern "C" fn send_running_log(p_sim_running_log: *mut sim_running_log) -> ::std::os::raw::c_int{
    return 0;
}

#[no_mangle]
pub extern "C" fn send_model_event(p_sim_model_event: *mut sim_model_event) -> ::std::os::raw::c_int{
    return 0;
}
