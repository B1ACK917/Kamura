use crate::router::auth;
use crate::router::payloads::{Arch, ArchList, AuthorizedPayload, CommonResponse, GetArchPayload, RawArch};
use axum::extract::State;
use axum::Json;
use kamura_operator::{Operator, Topology, Units};
use sayaka::debug_fn;

pub async fn flush_all(mut state: State<Operator>, Json(payload): Json<AuthorizedPayload>) -> Json<CommonResponse> {
    debug_fn!();
    if !auth(payload.auth) {
        return Json(CommonResponse { success: false, message: "Authorize Failed".to_string() });
    }
    match state.flush_all() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "Flushed All Redis".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

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
    match state.read_arch(payload.target) {
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
    match state.parse_arch(payload.target) {
        Ok(elements) => {
            Json(Arch { success: true, elements, message: "".to_string() })
        }
        Err(err) => {
            Json(Arch { success: false, elements: Vec::new(), message: err.to_string() })
        }
    }
}