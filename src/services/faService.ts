import {
    addStateFA,
    addTransitionFA,
    createNewFA, deleteFAApi, deleteFAResponse,
    generateInputs,
    multiRunStrFA,
    removeStateFA,
    removeTransitFA,
    runStrFA,
    updateStateFA,
    updateTransitFA,
} from "@/api/faAPI.ts";
import type {
    addStateFARequest,
    addStateFAResponse,
    addTransitFARequest,
    addTransitFAResponse,
    createNewFAResponse,
    deleteStateFARequest,
    deleteStateFAResponse,
    generateInputsRequest,
    generateInputsResponse,
    multiRunStrFARequest,
    multiRunStrFAResponse,
    removeTransitFARequest,
    removeTransitFAResponse,
    runStrFARequest,
    runStrFAResponse,
    updateStateFARequest,
    updateStateFAResponse,
    updateTransitFARequest,
    updateTransitFAResponse,
} from "@/api/faAPI.ts";
import { markDirty } from "@/services/dirtyState.ts";

export type {
    addStateFARequest,
    addStateFAResponse,
    addTransitFARequest,
    addTransitFAResponse,
    createNewFAResponse,
    deleteStateFARequest,
    deleteStateFAResponse,
    generateInputsRequest,
    generateInputsResponse,
    lineTest,
    multiRunStrFARequest,
    multiRunStrFAResponse,
    removeTransitFARequest,
    removeTransitFAResponse,
    RunStep,
    runStrFARequest,
    runStrFAResponse,
    Trace,
    updateStateFARequest,
    updateStateFAResponse,
    updateTransitFARequest,
    updateTransitFAResponse,
} from "@/api/faAPI.ts";

export const createFA = async (name: string): Promise<createNewFAResponse> => {
    const response = await createNewFA(name);
    if (response.status !== 200) {
        throw new Error(`createNewFA: status ${response.status}`);
    }
    markDirty(response.automaton.id, true);
    return response;
};

export const deleteFAService = async (automaton_id: number): Promise<deleteFAResponse> => {
    const response = await deleteFAApi(automaton_id);
    if (response.status !== 200) {
        throw new Error(`deleteFA: status ${response.status}`);
    }
    return response;
};

export const addState = async (params: addStateFARequest): Promise<addStateFAResponse> => {
    const response = await addStateFA(params);
    if (response.status !== 200) {
        throw new Error(`addState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const removeState = async (params: deleteStateFARequest): Promise<deleteStateFAResponse> => {
    const response = await removeStateFA(params);
    if (response.status !== 200) {
        throw new Error(`removeState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const updateState = async (params: updateStateFARequest): Promise<updateStateFAResponse> => {
    const response = await updateStateFA(params);
    if (response.status !== 200) {
        throw new Error(`updateState: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const addTransition = async (params: addTransitFARequest): Promise<addTransitFAResponse> => {
    const response = await addTransitionFA(params);
    if (response.status !== 200) {
        throw new Error(`addTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const updateTransition = async (params: updateTransitFARequest): Promise<updateTransitFAResponse> => {
    const response = await updateTransitFA(params);
    if (response.status !== 200) {
        throw new Error(`updateTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const removeTransitionService = async (params: removeTransitFARequest): Promise<removeTransitFAResponse> => {
    const response = await removeTransitFA(params);
    if (response.status !== 200) {
        throw new Error(`removeTransition: status ${response.status}`);
    }
    markDirty(params.automatonId, true);
    return response;
};

export const runString = async (params: runStrFARequest): Promise<runStrFAResponse> => {
    const response = await runStrFA(params);
    if (![ 200, 401, 402 ].includes(response.status)) {
        throw new Error(`runString: status ${response.status}`);
    }
    return response;
};

export const runMultipleStrings = async (params: multiRunStrFARequest): Promise<multiRunStrFAResponse> => {
    const response = await multiRunStrFA(params);
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