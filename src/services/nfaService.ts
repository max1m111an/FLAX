import {
    addStateNFA,
    addTransitionNFA,
    createNewNFA,
    generateInputs,
    multiRunStrNFA,
    removeStateNFA,
    removeTransitNFA,
    runStrNFA,
    updateStateNFA,
    updateTransitNFA,
} from "@/api/nfaAPI.ts";
import type {
    addStateNFARequest,
    addStateNFAResponse,
    addTransitNFARequest,
    addTransitNFAResponse,
    createNewNFAResponse,
    deleteStateNFARequest,
    deleteStateNFAResponse,
    generateInputsRequest,
    generateInputsResponse,
    multiRunStrNFARequest,
    multiRunStrNFAResponse,
    removeTransitNFARequest,
    removeTransitNFAResponse,
    runStrNFARequest,
    runStrNFAResponse,
    updateStateNFARequest,
    updateStateNFAResponse,
    updateTransitNFARequest,
    updateTransitNFAResponse,
} from "@/api/nfaAPI.ts";
import { markDirty } from "@/services/dirtyState.ts";

export type {
    addStateNFARequest,
    addStateNFAResponse,
    addTransitNFARequest,
    addTransitNFAResponse,
    createNewNFAResponse,
    deleteStateNFARequest,
    deleteStateNFAResponse,
    generateInputsRequest,
    generateInputsResponse,
    lineTest,
    multiRunStrNFARequest,
    multiRunStrNFAResponse,
    removeTransitNFARequest,
    removeTransitNFAResponse,
    RunStep,
    runStrNFARequest,
    runStrNFAResponse,
    Trace,
    updateStateNFARequest,
    updateStateNFAResponse,
    updateTransitNFARequest,
    updateTransitNFAResponse,
} from "@/api/nfaAPI.ts";

export const createNFA = async (name: string): Promise<createNewNFAResponse> => {
    const response = await createNewNFA(name);
    if (response.status !== 200) {
        throw new Error(`createNewNFA: status ${response.status}`);
    }
    markDirty(response.automaton.id, true);
    return response;
};

export const addState = async (params: addStateNFARequest): Promise<addStateNFAResponse> => {
    const response = await addStateNFA(params);
    if (response.status !== 200) {
        throw new Error(`addState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const removeState = async (params: deleteStateNFARequest): Promise<deleteStateNFAResponse> => {
    const response = await removeStateNFA(params);
    if (response.status !== 200) {
        throw new Error(`removeState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const updateState = async (params: updateStateNFARequest): Promise<updateStateNFAResponse> => {
    const response = await updateStateNFA(params);
    if (response.status !== 200) {
        throw new Error(`updateState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const addTransition = async (params: addTransitNFARequest): Promise<addTransitNFAResponse> => {
    const response = await addTransitionNFA(params);
    if (response.status !== 200) {
        throw new Error(`addTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const updateTransition = async (params: updateTransitNFARequest): Promise<updateTransitNFAResponse> => {
    const response = await updateTransitNFA(params);
    if (response.status !== 200) {
        throw new Error(`updateTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const removeTransition = async (params: removeTransitNFARequest): Promise<removeTransitNFAResponse> => {
    const response = await removeTransitNFA(params);
    if (response.status !== 200) {
        throw new Error(`removeTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const runString = async (params: runStrNFARequest): Promise<runStrNFAResponse> => {
    const response = await runStrNFA(params);
    if (![ 200, 401, 402 ].includes(response.status)) {
        throw new Error(`runString: status ${response.status}`);
    }
    return response;
};

export const runMultipleStrings = async (params: multiRunStrNFARequest): Promise<multiRunStrNFAResponse> => {
    const response = await multiRunStrNFA(params);
    if (response.status !== 200) {
        throw new Error(`runMultipleStrings: status ${response.status}`);
    }
    return response;
};

export const generateTestInputs = async (params: generateInputsRequest): Promise<generateInputsResponse> => {
    const response = await generateInputs(params);
    if (response.status !== 200) {
        throw new Error(`generateTestInputs: status ${response.status}`);
    }
    return response;
};