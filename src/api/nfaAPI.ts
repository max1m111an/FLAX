import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel, StateModel, TransitionModel } from "@/types/Automaton.ts";

type createNewNFAResponse = {
    status: number;
    message: string;
    automaton: AutomatonModel
}

export const createNewNFA = async (name: string): Promise<createNewNFAResponse> => {
    try {
        const response = await invoke<createNewNFAResponse>("create_new_nfa", { name });
        return response;
    } catch (error) {
        console.error("Ошибка при вызове create_new_nfa:", error);
        throw error;
    }
};

type addStateNFARequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}
type addStateNFAResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const addStateNFA = async (params: addStateNFARequest): Promise<addStateNFAResponse> => {
    try {
        const response = await invoke<addStateNFAResponse>("nfa_add_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_add_state:", error);
        throw error;
    }
};

type deleteStateNFARequest = {
    automatonId: number;
    stateId: number;
}
type deleteStateNFAResponse = {
    status: number;
    message: string;
}

export const removeStateNFA = async (params: deleteStateNFARequest): Promise<deleteStateNFAResponse> => {
    try {
        const response = await invoke<deleteStateNFAResponse>("nfa_remove_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_remove_state:", error);
        throw error;
    }
};

export type updateStateNFARequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
    isFinal?: boolean;
}
type updateStateNFAResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const updateStateNFA = async (params: updateStateNFARequest): Promise<updateStateNFAResponse> => {
    try {
        const response = await invoke<updateStateNFAResponse>("nfa_update_state", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_update_state:", error);
        throw error;
    }
};

type addTransitNFARequest = {
    automatonId: number;
    from: number;
    to: number;
    symbols: string[];
}

type addTransitNFAResponse = {
    status: number;
    message: string;
    transition: TransitionModel[];
}

export const addTransitionNFA = async (params: addTransitNFARequest): Promise<addTransitNFAResponse> => {
    try {
        const response = await invoke<addTransitNFAResponse>("nfa_add_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_add_transition:", error);
        throw error;
    }
};

type updateTransitNFARequest = {
    automatonId: number;
    transitionId: number;
    new_from?: number;
    new_to?: number;
    new_symbol?: string;
    new_label?: string | null;
}

type updateTransitNFAResponse = {
    status: number;
    message: string;
    transition: TransitionModel[];
}

export const updateTransitNFA = async (params: updateTransitNFARequest): Promise<updateTransitNFAResponse> => {
    try {
        const response = await invoke<updateTransitNFAResponse>("nfa_update_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_update_transition:", error);
        throw error;
    }
};

type removeTransitNFARequest = {
    automatonId: number;
    transitionId: number;
}

type removeTransitNFAResponse = {
    status: number;
    message: string;
}

export const removeTransitNFA = async (params: removeTransitNFARequest): Promise<removeTransitNFAResponse> => {
    try {
        const response = await invoke<removeTransitNFAResponse>("nfa_remove_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_remove_transition:", error);
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

type runStrNFARequest = {
    automatonId: number;
    input: string;
}

type runStrNFAResponse = {
    status: number;
    message: string;
    traces: Trace[];
}

export const runStrNFA = async (params: runStrNFARequest): Promise<runStrNFAResponse> => {
    try {
        const response = await invoke<runStrNFAResponse>("nfa_run_str", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове nfa_run_str:", error);
        throw error;
    }
};

