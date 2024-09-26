import {reactive} from "vue";

export const kamuraEngineUrl = "https://kamura-engine.malloc.fun"
export const topologyTemplate = {
    "hierarchy": {},
    "binding": []
}
export const viewerDefaultVars = {
    collapse: {
        left: false,
        right: false,
    },
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
    },
    archEditDialogVisible: false,
    archEditDialogForm: reactive({
        target: '',
        elements: '',
        topology: null,
    })
};

export const runnerDefaultTaskInfo = {
    arch: null,
    workload: null,
    submitTime: null,
    finishedTime: null,
    elapsed: null
};