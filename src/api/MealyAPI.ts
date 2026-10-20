import { AutomatonModel, StateModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { invoke } from "@tauri-apps/api/core";


export type createNewMealyResponse = {
    status: number;
    message: string;
    automaton: AutomatonModel
}

export const createNewMealy = async (name: string): Promise<createNewMealyResponse> => {
    try {
        return await invoke<createNewMealyResponse>("mealy_create_new", { name });
    } catch (error) {
        console.error("Ошибка при вызове mealy_create_new:", error);
        throw error;
    }
};


export type deleteMealyResponse = {
    status: number;
    message: string;
}

export const deleteMealyApi = async (automatonId: number): Promise<deleteMealyResponse> => {
    try {
        return await invoke<deleteMealyResponse>("mealy_remove_automaton", { automatonId });
    } catch (error) {
        console.error("Ошибка при вызове mealy_remove_automaton:", error);
        throw error;
    }
};


export type addStateMealyRequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}

export type addStateMealyResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const addStateFA = async (params: addStateMealyRequest): Promise<addStateMealyResponse> => {
    try {
        return await invoke<addStateMealyResponse>("mealy_add_state", params);
    } catch (error) {
        console.error("Ошибка при вызове fa_add_state:", error);
        throw error;
    }
};


export type updateStateMealyRequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
    isFinal?: boolean;
}

export type updateStateMealyResponse = {
    status: number;
    message: string;
    state: StateModel;
}

export const updateStateMealy = async (params: updateStateMealyRequest): Promise<updateStateMealyResponse> => {
    try {
        return await invoke<updateStateMealyResponse>("mealy_update_state", params);
    } catch (error) {
        console.error("Ошибка при вызове mealy_update_state:", error);
        throw error;
    }
};


export type removeStateMealyRequest = {
    automatonId: number;
    stateId: number;
}

export type removeStateMealyResponse = {
    status: number;
    message: string;
}

export const removeStateMealy = async (params: removeStateMealyRequest): Promise<removeStateMealyResponse> => {
    try {
        return await invoke<removeStateMealyResponse>("mealy_remove_state", params);
    } catch (error) {
        console.error("Ошибка при вызове mealy_remove_state:", error);
        throw error;
    }
};


export type addTransitMealyRequest = {
    automatonId: number;
    from: number;
    to: number;
    input: string;
    output: string;
}

export type addTransitMealyResponse = {
    status: number;
    message: string;
    transition: TransitionMealyModel[];
}

export const addTransitionMealy = async (params: addTransitMealyRequest): Promise<addTransitMealyResponse> => {
    try {
        const response = await invoke<addTransitMealyResponse>("mealy_add_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове mealy_add_transition:", error);
        throw error;
    }
};


export type updateTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
    input?: number;
    output?: number;
}

export type updateTransitMealyResponse = {
    status: number;
    message: string;
    transition: TransitionMealyModel[];
}

export const updateTransitMealy = async (params: updateTransitMealyRequest): Promise<updateTransitMealyResponse> => {
    try {
        const response = await invoke<updateTransitMealyResponse>("mealy_update_transition", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове mealy_update_transition:", error);
        throw error;
    }
};


export type removeTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
}

export type removeTransitMealyResponse = {
    status: number;
    message: string;
}

export const removeTransitMealy = async (params: removeTransitMealyRequest): Promise<removeTransitMealyResponse> => {
    try {
        return await invoke<removeTransitMealyResponse>("mealy_remove_transition", params);
    } catch (error) {
        console.error("Ошибка при вызове mealy_remove_transition:", error);
        throw error;
    }
};