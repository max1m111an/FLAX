import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel, StateModel } from "@/interface/Automaton.ts";

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