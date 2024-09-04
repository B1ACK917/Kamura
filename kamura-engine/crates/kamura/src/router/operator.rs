use crate::router::payloads::{Arch, ArchList, CommonResponse, GetArchPayload, RawArch, SaveArchPayload, Unit};
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
        Ok(topology) => {
            Json(RawArch { success: true, topology, message: "".to_string() })
        }
        Err(err) => {
            Json(RawArch { success: false, topology: Topology::new(), message: err.to_string() })
        }
    }
}

pub async fn get_units(state: State<Operator>) -> Json<Unit> {
    debug_fn!();
    match state.read_units() {
        Ok(units) => {
            Json(Unit { success: true, units, message: "".to_string() })
        }
        Err(err) => {
            Json(Unit { success: false, units: Units::new(), message: err.to_string() })
        }
    }
}

pub async fn get_arch(state: State<Operator>, Json(payload): Json<GetArchPayload>) -> Json<Arch> {
    debug_fn!();
    match state.parse_arch(payload.target, payload.reset) {
        Ok((topology, elements)) => {
            Json(Arch { success: true, topology, elements, message: "".to_string() })
        }
        Err(err) => {
            Json(Arch { success: false, topology: Topology::new(), elements: Vec::new(), message: err.to_string() })
        }
    }
}

pub async fn save_arch(state: State<Operator>, Json(payload): Json<SaveArchPayload>) -> Json<CommonResponse> {
    debug_fn!();
    match state.save_arch(payload.target, payload.topology, payload.elements) {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}