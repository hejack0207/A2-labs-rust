use std::ffi::CStr;

include!("kafka-bindings.rs");

#[no_mangle]
pub extern "C" fn send_sim_status_reply(
        p_sim_status_reply: *mut sim_status_reply,
) -> ::std::os::raw::c_int{
    println!("send_sim_status_reply");
    unsafe{
        println!("header->simulationId: {}", CStr::from_ptr((*p_sim_status_reply).header.simulationId).to_str().expect("should be converted to String"));
        println!("containerId: {}", CStr::from_ptr((*p_sim_status_reply).containerId).to_str().expect("should be converted to String"));
        println!("containerIp: {}", CStr::from_ptr((*p_sim_status_reply).containerIp).to_str().expect("should be converted to String"));
        println!("port: {}", (*p_sim_status_reply).port);
    }
    return 0;
}

pub extern "C" fn send_sim_status_data(p_sim_status_data: *mut sim_status_data) -> ::std::os::raw::c_int{
    return 0;
}

pub extern "C" fn send_running_log(p_sim_running_log: *mut sim_running_log) -> ::std::os::raw::c_int{
    return 0;
}
