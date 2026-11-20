import { call } from "@/api/call.ts";
import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { AutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { MultiRunStrMealyResponse, RunStrMealyTracesResponse } from "@/types/api/mealy.ts";
import { MealyStatePayload } from "@/types/Payloads.ts";

/**
 * Сервис для работы с машинами Мили (MEALY).
 * Инкапсулирует разбор пар вход/выход, нормализацию трасс и вызов команд бэкенда.
 */
export const mealyService: IAutomatonService<
    AutomatonModel,
    StateMealyModel,
    TransitionMealyModel,
    MealyStatePayload,
    RunStrMealyTracesResponse,
    MultiRunStrMealyResponse
> = {
    async createAutomaton (name: string) {
        const response = await call("mealy_create_new", { name });
        markDirty(response.automaton.id, true);
        return response;
    },

    async deleteAutomaton (automatonId: number) {
        await call("mealy_remove_automaton", { automatonId });
    },

    async addState (automatonId: number, payload: MealyStatePayload) {
        if (
            payload.label === undefined ||
            payload.x === undefined ||
            payload.y === undefined ||
            payload.isInitial === undefined
        ) {
            throw new Error("Не заполнены обязательные поля для создания состояния машины Мили");
        }

        const response = await call("mealy_add_state", {
            automatonId,
            label: payload.label,
            x: payload.x,
            y: payload.y,
            isInitial: payload.isInitial,
        });
        markDirty(automatonId, true);
        return response;
    },

    async removeState (automatonId: number, stateId: number) {
        await call("mealy_remove_state", { automatonId, stateId });
        markDirty(automatonId, true);
    },

    async updateState (automatonId: number, stateId: number, payload: MealyStatePayload) {
        const response = await call("mealy_update_state", {
            automatonId,
            stateId,
            label: payload.label,
            x: payload.x,
            y: payload.y,
            isInitial: payload.isInitial,
        });
        markDirty(automatonId, true);
        return response;
    },

    async addTransition (automatonId: number, from: number, to: number, symbolStr: string) {
        const chunks = symbolStr.trim().split(/\s+/);
        const newTransitions: TransitionMealyModel[] = [];

        for (const chunk of chunks) {
            const parts = chunk.split(/[→\-,;]/);
            const input = parts.length >= 2 ? parts[0] : (chunk.length === 2 ? chunk[0] : undefined);
            const output = parts.length >= 2 ? parts[1] : (chunk.length === 2 ? chunk[1] : undefined);

            if (input && output) {
                const response = await call("mealy_add_transition", {
                    automatonId,
                    from,
                    to,
                    input,
                    output,
                });
                newTransitions.push(...response.transition);
            }
        }

        markDirty(automatonId, true);
        return { status: 200, message: "OK", transition: newTransitions };
    },

    async removeTransition (automatonId: number, transitionId: number) {
        await call("mealy_remove_transition", { automatonId, transitionId });
        markDirty(automatonId, true);
    },

    async runStr (automatonId: number, input: string): Promise<RunStrMealyTracesResponse> {
        const response = await call("mealy_run_str", { automatonId, input });
        return {
            status: response.status,
            message: response.message,
            traces: [ { steps: response.steps, isFinal: response.status === 200 } ],
        };
    },

    async runMultipleStr (automatonId: number, inputs: string[]) {
        return await call("mealy_multi_run", { automatonId, inputs });
    },

    async generateTestInputs (automatonId: number) {
        return await call("mealy_generate_inputs", { automatonId });
    },
};