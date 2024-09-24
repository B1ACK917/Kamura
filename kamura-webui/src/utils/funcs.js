import axios from "axios";
import {kamuraEngineUrl} from "@/utils/consts";

function addNewModule(unitName, unitType, units, elements, nodeGap = [10, 10]) {
    let [x, y] = [0, 0];
    const [nodeGapX, nodeGapY] = nodeGap;
    console.log(units);
    console.log(unitType);
    console.log(units[unitType]);
    const ports = units[unitType]["ports"];
    elements.push({data: {id: unitName, label: unitName}, classes: 'subgraph'});
    for (const port of ports["in_ports"]) {
        const data = {
            data: {
                id: `${unitName}.ports.${port}`,
                label: port,
                parent: unitName
            },
            position: {x, y}
        };
        // console.log(data);
        elements.push(data);
        x += nodeGapX;
    }
    x = 0;
    y += nodeGapY;

    for (const port of ports["out_ports"]) {
        const data = {
            data: {
                id: `${unitName}.ports.${port}`,
                label: port,
                parent: unitName
            },
            position: {x, y}
        };
        elements.push(data);
        x += nodeGapX;
    }
    return elements;
}

export function addAUnit(unitName, unitType, units, topology, elements) {
    if (unitType in topology["instances"]) {
        topology["instances"][unitType].push(unitName);
    } else {
        topology["instances"][unitType] = [unitName];
    }
    console.log(topology);
    elements = addNewModule(unitName, unitType, units, elements, [400, 100]);
    return elements;
}

export function checkBinding(bindingStack, nodeID) {
    if (nodeID.includes('.')) {
        if (bindingStack != null) {
            if (bindingStack.endsWith("out") && nodeID.endsWith("out")) {
                return [false, "Trying to bind two OUT ports!"];
            }
            if (bindingStack.endsWith("in") && nodeID.endsWith("in")) {
                return [false, "Trying to bind two IN ports!"];
            }
        }
        return [true, ""];
    } else {
        return [false, "Looks like not a port?"];
    }
}

export function updateBinding(bindingStack, nodeID, topology, elements, popBinding = true) {
    const newEdge = [];
    if (bindingStack) {
        let [source, target] = [bindingStack, nodeID];
        if (nodeID.endsWith("out")) {
            [source, target] = [nodeID, bindingStack];
        }

        topology["binding"].push({
            "source": source,
            "target": target
        });
        newEdge.push({
            data: {
                id: `${source}-${target}`,
                source: `${source}`,
                target: `${target}`
            }
        });
        elements.push({
            data: {
                id: `${source}-${target}`,
                source: `${source}`,
                target: `${target}`
            }
        });
        if (popBinding)
            bindingStack = null;
    } else {
        bindingStack = nodeID;
    }
    return [bindingStack, newEdge, topology, elements];
}

export function parseUnits(data) {
    const orderedUnits = Object.keys(data.units.units).sort().reduce(
        (obj, key) => {
            obj[key] = data.units[key];
            return obj;
        },
        {}
    );
    return [data.units, orderedUnits];
}

export function parseArch(data) {
    const cyElements = data.elements;
    const topology = data.topology;
    const instances = [];
    for (let part in data.topology["hierarchy"]["core0"]) {
        for (let instances_ of data.topology["hierarchy"]["core0"][part]) {
            for (let unit in instances_) {
                instances.push(...Object.keys(instances_[unit]));
            }
        }
    }
    instances.sort();
    return [cyElements, topology, instances]
}

export async function commonFetchGet(url) {
    const response = await axios.get(url);
    const data = response.data;
    if (!data.success) throw data.message;
    return data;
}

export async function commonFetchPost(url, payload) {
    const response = await axios.post(url, payload);
    const data = response.data;
    if (!data.success) throw data.message;
    return data;
}