#[allow(unused_imports)]
use axum::{
    extract,
    extract::Json,
    routing::get,
    routing::post,
    Router,
    http::StatusCode,
};


use std::ffi::CString;


#[allow(non_snake_case, non_camel_case_types)]
#[allow(dead_code)]
mod model{
    use serde::Deserialize;
    use serde_with::serde_as;

    #[derive(Deserialize)]
    pub struct model_init_request {
        pub header: sim_msg_header,
        #[serde(rename = "body")]
        // pub params: model_init_params<33>,
        pub params: model_init_params,
    }

    #[derive(Deserialize)]
    pub struct sim_msg_header {
        pub simulationId: String,
        pub sceneObjectId: String,
        pub simulationTime: ::std::os::raw::c_long,
        pub timestamp: ::std::os::raw::c_long,
    }

    #[serde_as]
    #[derive(Deserialize)]
    pub struct model_init_params {
        pub ueid: u16,
        pub sst: u8,
        pub sd: u32,
        pub cyclicPrefix: u8,
        pub subCarrierSpacing: u8,
        pub routeAddIp: String,
        pub usimMode: u8,
        pub authAlgo: u8,
        pub opType: u8,
        pub opcOrOp: String,
        pub k: String,
        pub imsi: String,
        pub imei: String,
        pub msisdn: String,
        pub imeisv: String,
        pub dnn: String,
        pub latitude: ::std::os::raw::c_int,
        pub longitude: ::std::os::raw::c_int,
        pub altitude: u32,
    }

    pub const CONTROL_TYPE_START: CONTROL_TYPE = 1;
    pub const CONTROL_TYPE_PAUSE: CONTROL_TYPE = 2;
    pub const CONTROL_TYPE_RESUME: CONTROL_TYPE = 3;
    pub const CONTROL_TYPE_DOUBLESPEED: CONTROL_TYPE = 4;
    pub const CONTROL_TYPE_STOP: CONTROL_TYPE = 5;
    #[doc = " kafka 消息"]
    pub type CONTROL_TYPE = ::std::os::raw::c_uint;

    #[derive(Deserialize)]
    pub struct sim_control {
        pub header: sim_msg_header,
        pub sceneObjectId: String,
        pub simulationTime: ::std::os::raw::c_long,
        pub controlType: u8,
        pub speed: u8,
    }

    #[derive(Deserialize)]
    pub struct model_config_params {
        pub optType: u8,
        pub capacity: u32,
        pub serviceAddr: String,
        pub phoneNum: String,
    }

    #[derive(Deserialize)]
    pub struct model_simfault_params {
        pub simFault: u8,
        pub startError: u8,
        pub outOfSync: u8,
    }

    #[derive(Deserialize)]
    pub struct model_config_request {
        pub header: sim_msg_header,
        // pub __bindgen_anon_1: model_config_request__bindgen_ty_1,
        pub configOrSimfault: ::std::os::raw::c_int,
    }

    // #[derive(Deserialize)]
    // pub union model_config_request__bindgen_ty_1 {
    //     pub configParams: model_config_params,
    //     pub simfaultParams: model_simfault_params,
    // }
}

use model::*;

use uesim::{
    ue_sim_api::{
        on_model_init,
    },
    model_config,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    println!("web server about to start!");
    let app = Router::new()
        .route("/", get(|| async { println!("I got it"); "Hello, World!" }))
        .route("/model/init", get(web_model_init).post(web_model_init))
        .route("/model/config", get(web_model_config).post(web_model_config));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("web server about to quit!");
}

fn get_last_n_elements<const N: usize>(vec: &Vec<u8>) -> [i8; N] {
    let len = vec.len();
    if len < N {
        panic!("Vector must have at least {} elements.", N - 1);
    }

    let mut result: [i8; N] = [0; N];
    let slice = &vec[len - N ..len]; // 获取最后 N 个元素的切片

    for (index, &value) in slice.iter().enumerate() {
        result[index] = value as i8; // 将 u8 转换为 i8
    }

    result
}

async fn web_model_init(Json(payload): Json<model_init_request>)->Result<String, StatusCode>{
    println!("web_model_init");
    let mut request :uesim::ue_sim_api::model_init_request = uesim::ue_sim_api::model_init_request{
        header: uesim::ue_sim_api::sim_msg_header {
            simulationId: CString::new(payload.header.simulationId).expect("should be converted to c string").into_raw(),
            sceneObjectId:  CString::new(payload.header.sceneObjectId).expect("should be converted to c string").into_raw(),
            simulationTime: payload.header.simulationTime,
            timestamp: payload.header.timestamp,
        },
        params: uesim::ue_sim_api::model_init_params {
            ueid: payload.params.ueid,
            sst: payload.params.sst,
            sd: payload.params.sd,
            cyclicPrefix: payload.params.cyclicPrefix,
            subCarrierSpacing: payload.params.subCarrierSpacing,
            routeAddIp: CString::new(payload.params.routeAddIp).expect("should be converted to c string").into_raw(),
            usimMode: payload.params.usimMode,
            authAlgo: payload.params.authAlgo,
            opType: payload.params.opType,
            // opcOrOp: payload.params.opcOrOp,
            // k: payload.params.k,
            // imsi: payload.params.imsi,
            // imei: payload.params.imei,
            // msisdn: payload.params.msisdn,
            // imeisv: payload.params.imeisv,
            opcOrOp: get_last_n_elements::<33>(&CString::new(payload.params.opcOrOp).expect("should be a c string").into_bytes_with_nul()),
            k: get_last_n_elements::<33>(&CString::new(payload.params.k).expect("should be a c string").into_bytes_with_nul()),
            imsi: get_last_n_elements::<16>(&CString::new(payload.params.imsi).expect("should be a c string").into_bytes_with_nul()),
            imei: get_last_n_elements::<16>(&CString::new(payload.params.imei).expect("should be a c string").into_bytes_with_nul()),
            msisdn: get_last_n_elements::<12>(&CString::new(payload.params.msisdn).expect("should be a c string").into_bytes_with_nul()),
            imeisv: get_last_n_elements::<17>(&CString::new(payload.params.imeisv).expect("should be a c string").into_bytes_with_nul()),
            dnn: CString::new(payload.params.dnn).expect("should be converted to c string").into_raw(),
            latitude: payload.params.latitude,
            longitude: payload.params.longitude,
            altitude: payload.params.altitude,
        },
    };
    unsafe {
        on_model_init(std::ptr::addr_of_mut!(request));
    }
    Ok("200".to_string())

async fn web_model_config()->Result<String, StatusCode>{
    Ok(model_config())
}
