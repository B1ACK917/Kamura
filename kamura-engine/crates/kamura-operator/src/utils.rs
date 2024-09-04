use crate::{Topology, Units};
use kamura_core::consts::{OPERATOR_DEFAULT_MODULE_GAP, OPERATOR_DEFAULT_MODULE_PER_LINE, OPERATOR_DEFAULT_NODE_GAP};
use serde_json::{Result, Value};
use std::cmp::max;
use sayaka::debug_fn;

pub(crate) fn convert_to_cy_elements(units: &Units, topology: &Topology) -> Result<Vec<Value>> {
    debug_fn!();
    let mut elements: Vec<Value> = Vec::new();
    elements = add_subgraph(units, topology, elements)?;
    elements = add_ports(units, topology, elements)?;
    elements = add_edges(topology, elements)?;
    Ok(elements)
}

pub(crate) fn add_subgraph(units: &Units, topology: &Topology, mut elements: Vec<Value>) -> Result<Vec<Value>> {
    debug_fn!();
    for (module, instances) in &topology.instances {
        for instance in instances {
            let _basic_unit = &units[module];
            let data = format!(r#"{{
                "data": {{
                    "id": "{instance}",
                    "label": "{instance}"
                }},
                "classes": "subgraph"
            }}"#
            );
            elements.push(serde_json::from_str(data.as_str())?);
        }
    }
    Ok(elements)
}

pub(crate) fn add_ports(units: &Units, topology: &Topology, mut elements: Vec<Value>) -> Result<Vec<Value>> {
    debug_fn!();
    let (mut x, mut y) = (0, 0);
    // let num_modules = topology.instances.len() as f32;
    // let col_instance = num_modules.sqrt().ceil() as u32;
    let col_instance = OPERATOR_DEFAULT_MODULE_PER_LINE;
    let mut n_instance = 0;

    for (module, instances) in &topology.instances {
        for instance in instances {
            let (mut cur_x, mut cur_y) = (x, y);
            let unit = &units[module];
            let mut max_x = 0;

            for port in &unit.ports.in_ports {
                let data = format!(r#"{{
                    "data": {{
                      "id": "{instance}.ports.{port}",
                      "label": "{port}",
                      "parent": "{instance}"
                    }},
                    "position": {{ "x": {cur_x}, "y": {cur_y} }}
                  }}"#
                );
                elements.push(serde_json::from_str(data.as_str())?);
                cur_x += OPERATOR_DEFAULT_NODE_GAP.0;
            }
            max_x = max(max_x, cur_x);

            cur_x = x;
            cur_y += OPERATOR_DEFAULT_NODE_GAP.1;

            for port in &unit.ports.out_ports {
                let data = format!(r#"{{
                    "data": {{
                      "id": "{instance}.ports.{port}",
                      "label": "{port}",
                      "parent": "{instance}"
                    }},
                    "position": {{ "x": {cur_x}, "y": {cur_y} }}
                  }}"#
                );
                elements.push(serde_json::from_str(data.as_str())?);
                cur_x += OPERATOR_DEFAULT_NODE_GAP.0;
            }
            max_x = max(max_x, cur_x);

            n_instance += 1;
            x = max_x + OPERATOR_DEFAULT_MODULE_GAP.0;
            if n_instance == col_instance {
                y = cur_y + OPERATOR_DEFAULT_MODULE_GAP.1;
                (n_instance, x) = (0, 0);
            }
        }
    }
    Ok(elements)
}

pub(crate) fn add_edges(topology: &Topology, mut elements: Vec<Value>) -> Result<Vec<Value>> {
    debug_fn!();
    for edge in &topology.binding {
        let [s, t] = [edge.source.clone(), edge.target.clone()];
        let data = format!(r#"{{
                "data": {{
                    "id": "{s}-{t}",
                    "source": "{s}",
                    "target": "{t}"
                }}
            }}"#
        );
        elements.push(serde_json::from_str(data.as_str())?);
    }
    Ok(elements)
}