use crate::router::payloads::{Arch, ArchList, CommonResponse, GetArchPayload, RawArch, SaveArchPayload};
use axum::extract::State;
use axum::Json;
use kamura_operator::{Operator, Topology, Units};
use sayaka::debug_fn;


pub async fn list_arches(state: State<Operator>) -> Json<ArchList> {
    debug_fn!();
    match state.list_arches() {
        Ok(arches) => {
            Json(ArchList { success: true, arches, message: "".to_string() })
        }
        Err(err) => {
            Json(ArchList { success: false, arches: Vec::new(), message: err.to_string() })
        }
    }
}

pub async fn get_raw_arch(state: State<Operator>, Json(payload): Json<GetArchPayload>) -> Json<RawArch> {
    debug_fn!();
    match state.read_arch(&payload.target) {
        Ok((units, topology)) => {
            Json(RawArch { success: true, units, topology, message: "".to_string() })
        }
        Err(err) => {
            Json(RawArch { success: false, units: Units::new(), topology: Topology::new(), message: err.to_string() })
        }
    }
}

pub async fn get_arch(state: State<Operator>, Json(payload): Json<GetArchPayload>) -> Json<Arch> {
    debug_fn!();
    match state.parse_arch(payload.target, payload.reset) {
        Ok((units, topology, elements)) => {
            Json(Arch { success: true, units, topology, elements, message: "".to_string() })
        }
        Err(err) => {
            Json(Arch { success: false, units: Units::new(), topology: Topology::new(), elements: Vec::new(), message: err.to_string() })
        }
    }
}

pub async fn save_arch(state: State<Operator>, Json(payload): Json<SaveArchPayload>) -> Json<CommonResponse> {
    debug_fn!();
    match state.save_arch(payload.target, payload.units, payload.topology, payload.elements) {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}