import { invoke } from "@tauri-apps/api/core";
import {
    addStateMealyRequest,
    addStateMealyResponse, addTransitMealyRequest, addTransitMealyResponse,
    createMealyResponse,
    deleteMealyResponse, removeStateMealyRequest, removeStateMealyResponse,
    removeTransitMealyRequest, removeTransitMealyResponse, updateStateMealyRequest,
    updateStateMealyResponse, updateTransitMealyRequest, updateTransitMealyResponse,
} from "@/types/api/mealy.ts";


export const mealyApi = {
    async createAutomaton (name: string): Promise<createMealyResponse> {
        try {
            return await invoke<createMealyResponse>("mealy_create_new", { name });
        } catch (error) {
            console.error("Ошибка при вызове mealy_create_new:", error);
            throw error;
        }
    },
    async deleteAutomaton (automatonId: number): Promise<deleteMealyResponse> {
        try {
            return await invoke<deleteMealyResponse>("mealy_remove_automaton", { automatonId });
        } catch (error) {
            console.error("Ошибка при вызове mealy_remove_automaton:", error);
            throw error;
        }
    },
    async addState (params: addStateMealyRequest): Promise<addStateMealyResponse> {
        try {
            return await invoke<addStateMealyResponse>("mealy_add_state", params);
        } catch (error) {
            console.error("Ошибка при вызове fa_add_state:", error);
            throw error;
        }
    },
    async updateState (params: updateStateMealyRequest): Promise<updateStateMealyResponse> {
        try {
            return await invoke<updateStateMealyResponse>("mealy_update_state", params);
        } catch (error) {
            console.error("Ошибка при вызове mealy_update_state:", error);
            throw error;
        }
    },
    async removeState (params: removeStateMealyRequest): Promise<removeStateMealyResponse> {
        try {
            return await invoke<removeStateMealyResponse>("mealy_remove_state", params);
        } catch (error) {
            console.error("Ошибка при вызове mealy_remove_state:", error);
            throw error;
        }
    },
    async addTransitionMealy (params: addTransitMealyRequest): Promise<addTransitMealyResponse> {
        try {
            const response = await invoke<addTransitMealyResponse>("mealy_add_transition", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове mealy_add_transition:", error);
            throw error;
        }
    },
    async updateTransitMealy (params: updateTransitMealyRequest): Promise<updateTransitMealyResponse> {
        try {
            const response = await invoke<updateTransitMealyResponse>("mealy_update_transition", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове mealy_update_transition:", error);
            throw error;
        }
    },
    async removeTransitMealy (params: removeTransitMealyRequest): Promise<removeTransitMealyResponse> {
        try {
            return await invoke<removeTransitMealyResponse>("mealy_remove_transition", params);
        } catch (error) {
            console.error("Ошибка при вызове mealy_remove_transition:", error);
            throw error;
        }
    },
};