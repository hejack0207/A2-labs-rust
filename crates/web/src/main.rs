use axum::{
    routing::get,
    routing::post,
    Router,
    http::StatusCode,
};

use std::mem;

use axum::{
    extract,
};

use serde::Deserialize;
use serde_with::serde_as;


#[derive(Deserialize)]
struct model_init_request {
    pub header: sim_msg_header,
    pub params: model_init_params<33>,
}

#[derive(Deserialize)]
pub struct sim_msg_header {
    pub simulationId: Box<[i8]>,
    pub sceneObjectId: Box<[i8]>,
    pub simulationTime: ::std::os::raw::c_long,
    pub timestamp: ::std::os::raw::c_long,
}

#[serde_as]
#[derive(Deserialize)]
pub struct model_init_params<const N: usize> {
    pub ueid: u16,
    pub sst: u8,
    pub sd: u32,
    pub cyclicPrefix: u8,
    pub subCarrierSpacing: u8,
    pub routeAddIp:  Box<[i8]>,
    pub usimMode: u8,
    pub authAlgo: u8,
    pub opType: u8,
    #[serde_as(as = "[_;N]")]
    pub opcOrOp: [::std::os::raw::c_char; N],
    #[serde_as(as = "[_;N]")]
    pub k: [::std::os::raw::c_char; N],
    pub imsi: [::std::os::raw::c_char; 16usize],
    pub imei: [::std::os::raw::c_char; 16usize],
    pub msisdn: [::std::os::raw::c_char; 12usize],
    pub imeisv: [::std::os::raw::c_char; 17usize],
    pub dnn: Box<[i8]>,
    pub latitude: ::std::os::raw::c_int,
    pub longitude: ::std::os::raw::c_int,
    pub altitude: u32,
}

use uesim::{
    ue_sim_api::{
        on_model_init,
    },
    model_config,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let _ = app.clone().route("/model/init", get(web_model_init));
    let _ = app.clone().route("/model/config", get(web_model_config));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn web_model_init(extract::Json(payload): extract::Json<model_init_request >)->Result<String, StatusCode>{
    let request :uesim::ue_sim_api::model_init_request = uesim::ue_sim_api::model_init_request{
        header: uesim::ue_sim_api::sim_msg_header {
            simulationId: unsafe {
                std::ptr::read(Box::into_raw(payload.header.simulationId)).as_mut_ptr()
            },
            sceneObjectId: payload.header.sceneObjectId,
            simulationTime: payload.header.simulationTime,
            timestamp: payload.header.timestamp,
        },
        params: uesim::ue_sim_api::model_init_params {
            ueid: payload.params.ueid,
            sst: payload.params.sst,
            sd: payload.params.sd,
            cyclicPrefix: payload.params.cyclicPrefix,
            subCarrierSpacing: payload.params.subCarrierSpacing,
            routeAddIp: payload.params.routeAddIp,
            usimMode: payload.params.usimMode,
            authAlgo: payload.params.authAlgo,
            opType: payload.params.opType,
            opcOrOp: payload.params.opcOrOp,
            k: payload.params.k,
            imsi: payload.params.imsi,
            imei: payload.params.imei,
            msisdn: payload.params.msisdn,
            imeisv: payload.params.imeisv,
            dnn: payload.params.dnn,
            latitude: payload.params.latitude,
            longitude: payload.params.longitude,
            altitude: payload.params.altitude,
        },
    };
    unsafe {
        on_model_init(std::ptr::addr_of_mut!(request));
    }
    Ok("".to_string())
}

async fn web_model_config()->Result<String, StatusCode>{
    Ok(model_config())
}


