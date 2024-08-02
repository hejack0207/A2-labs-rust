use std::mem;
use crate::ue_sim_api;
pub use crate::ue_sim_api::model_config_request;

pub fn model_config()->String{
    let mut request : *mut model_config_request  = unsafe { mem::uninitialized() };
    unsafe { ue_sim_api::on_model_config(request) };
    "".to_string()
}

