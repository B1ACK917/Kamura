export const kamuraEngineUrl = "https://kamura-engine.malloc.fun"
export const topologyTemplate = {
    "hierarchy": {},
    "binding": []
}
export const viewerDefaultVars = {
    leftCollapse: false,
    rightCollapse: false,
    arches: null,
    selectedArch: null,
    raw: {
        units: null,
        topology: null
    },
    eval: {
        orderedUnits: null,
        instances: null,
    },
    cy: null,
    cyElements: [],
    removedNodes: [],
    mode: "view",
    bindingStack: null,
    lastInformation: {
        ins: [],
        outs: [],
        from: null,
        to: null,
    }
};