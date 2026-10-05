import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel, StateModel, TransitionModel } from "@/types/Automaton.ts";

export type createNewFAResponse = {
    status: number;
    message: string;
    automaton: AutomatonModel
}

export const createNewFA = async (name: string): Promise<createNewFAResponse> => {
    try {
        const response = await invoke<createNewFAResponse>("fa_create_new", { name });
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_create_new:", error);
        throw error;
    }
};

export type deleteFAResponse = {
    status: number;
    message: string;
}

export const deleteFAApi = async (automatonId: number): Promise<deleteFAResponse> => {
    try {
        const response = await invoke<deleteFAResponse>("fa_remove_automaton", { automatonId });
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_remove_automaton:", error);
        throw error;
    }
};

export type addStateFARequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}
export type addStateFAResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const addStateFA = async (params: addStateFARequest): Promise<addStateFAResponse> => {
    try {
        const response = await invoke<addStateFAResponse>("fa_add_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_add_state:", error);
        throw error;
    }
};

export type deleteStateFARequest = {
    automatonId: number;
    stateId: number;
}
export type deleteStateFAResponse = {
    status: number;
    message: string;
}

export const removeStateFA = async (params: deleteStateFARequest): Promise<deleteStateFAResponse> => {
    try {
        const response = await invoke<deleteStateFAResponse>("fa_remove_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_remove_state:", error);
        throw error;
    }
};

export type updateStateFARequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
    isFinal?: boolean;
}
export type updateStateFAResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const updateStateFA = async (params: updateStateFARequest): Promise<updateStateFAResponse> => {
    try {
        const response = await invoke<updateStateFAResponse>("fa_update_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_update_state:", error);
        throw error;
    }
};

export type addTransitFARequest = {
    automatonId: number;
    from: number;
    to: number;
    symbols: string[];
}

export type addTransitFAResponse = {
    status: number;
    message: string;
    transition: TransitionModel[];
}

export const addTransitionFA = async (params: addTransitFARequest): Promise<addTransitFAResponse> => {
    try {
        const response = await invoke<addTransitFAResponse>("fa_add_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_add_transition:", error);
        throw error;
    }
};

export type updateTransitFARequest = {
    automatonId: number;
    transitionId: number;
    newFrom?: number;
    newTo?: number;
    newSymbol?: string;
}

export type updateTransitFAResponse = {
    status: number;
    message: string;
    transition: TransitionModel[];
}

export const updateTransitFA = async (params: updateTransitFARequest): Promise<updateTransitFAResponse> => {
    try {
        const response = await invoke<updateTransitFAResponse>("fa_update_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_update_transition:", error);
        throw error;
    }
};

export type removeTransitFARequest = {
    automatonId: number;
    transitionId: number;
}

export type removeTransitFAResponse = {
    status: number;
    message: string;
}

export const removeTransitFA = async (params: removeTransitFARequest): Promise<removeTransitFAResponse> => {
    try {
        const response = await invoke<removeTransitFAResponse>("fa_remove_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_remove_transition:", error);
        throw error;
    }
};

export type RunStep = {
    from: number;
    symbol: string;
    to: number;
}
export type Trace = {
    steps: RunStep[];
    isFinal: boolean;
}

export type runStrFARequest = {
    automatonId: number;
    input: string;
}

export type runStrFAResponse = {
    status: number;
    message: string;
    traces: Trace[];
}

export const runStrFA = async (params: runStrFARequest): Promise<runStrFAResponse> => {
    try {
        const response = await invoke<runStrFAResponse>("fa_run_str", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_run_str:", error);
        throw error;
    }
};

export type lineTest = {
    line: string;
    isFinal: boolean;
    correctSymbols: number;
}

export type multiRunStrFARequest = {
    automatonId: number;
    inputs: string[];
}

export type multiRunStrFAResponse = {
    status: number;
    message: string;
    traces: lineTest[];
}

export const multiRunStrFA = async (params: multiRunStrFARequest): Promise<multiRunStrFAResponse> => {
    try {
        const response = await invoke<multiRunStrFAResponse>("fa_multi_run_str", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_multi_run_str:", error);
        throw error;
    }
};
export type generateInputsRequest = {
    automatonId: number;
}

export type generateInputsResponse = {
    status: number;
    message: string;
    inputs: string[];
}

export const generateInputs = async (params: generateInputsRequest): Promise<generateInputsResponse> => {
    try {
        const response = await invoke<generateInputsResponse>("fa_generate_inputs", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_generate_inputs:", error);
        throw error;
    }
};
