import { invoke } from "@tauri-apps/api/core";
import {
    addStateFARequest,
    addStateFAResponse,
    addTransitFARequest,
    addTransitFAResponse,
    createFAResponse,
    deleteFAResponse, generateInputsRequest, generateInputsResponse, multiRunStrFARequest, multiRunStrFAResponse,
    removeStateFARequest,
    removeStateFAResponse,
    removeTransitFARequest,
    removeTransitFAResponse, runStrFARequest, runStrFAResponse,
    updateStateFARequest,
    updateStateFAResponse,
    updateTransitFARequest,
    updateTransitFAResponse,
} from "@/types/api/fa.ts";


export const faApi = {
    async createAutomaton (name: string): Promise<createFAResponse> {
        try {
            const response = await invoke<createFAResponse>("fa_create_new", { name });
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_create_new:", error);
            throw error;
        }
    },
    async deleteAutomaton (automatonId: number): Promise<deleteFAResponse> {
        try {
            const response = await invoke<deleteFAResponse>("fa_remove_automaton", { automatonId });
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_remove_automaton:", error);
            throw error;
        }
    },
    async addState (params: addStateFARequest): Promise<addStateFAResponse> {
        try {
            const response = await invoke<addStateFAResponse>("fa_add_state", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_add_state:", error);
            throw error;
        }
    },
    async removeState (params: removeStateFARequest): Promise<removeStateFAResponse> {
        try {
            const response = await invoke<removeStateFAResponse>("fa_remove_state", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_remove_state:", error);
            throw error;
        }
    },
    async updateState (params: updateStateFARequest): Promise<updateStateFAResponse> {
        try {
            const response = await invoke<updateStateFAResponse>("fa_update_state", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_update_state:", error);
            throw error;
        }
    },
    async addTransition (params: addTransitFARequest): Promise<addTransitFAResponse> {
        try {
            const response = await invoke<addTransitFAResponse>("fa_add_transition", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_add_transition:", error);
            throw error;
        }
    },
    async updateTransit (params: updateTransitFARequest): Promise<updateTransitFAResponse> {
        try {
            const response = await invoke<updateTransitFAResponse>("fa_update_transition", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_update_transition:", error);
            throw error;
        }
    },
    async removeTransit (params: removeTransitFARequest): Promise<removeTransitFAResponse> {
        try {
            const response = await invoke<removeTransitFAResponse>("fa_remove_transition", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_remove_transition:", error);
            throw error;
        }
    },
    async runStr (params: runStrFARequest): Promise<runStrFAResponse> {
        try {
            const response = await invoke<runStrFAResponse>("fa_run_str", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_run_str:", error);
            throw error;
        }
    },
    async multiRunStr (params: multiRunStrFARequest): Promise<multiRunStrFAResponse> {
        try {
            const response = await invoke<multiRunStrFAResponse>("fa_multi_run_str", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_multi_run_str:", error);
            throw error;
        }
    },
    async generateInputs (params: generateInputsRequest): Promise<generateInputsResponse> {
        try {
            const response = await invoke<generateInputsResponse>("fa_generate_inputs", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_generate_inputs:", error);
            throw error;
        }
    },
};

