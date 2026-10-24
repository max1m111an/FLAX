import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { AutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { StatePayload, TransitionPayload } from "@/types/Payloads.ts";
import { multiRunStrFAResponse, runStrFAResponse } from "@/types/api/fa.ts";
import { mealyApi } from "@/api/MealyAPI.ts";

// @ts-expect-error: todo: add insufficient Api
export const mealyService: IAutomatonService<
    AutomatonModel,
    StateMealyModel,
    TransitionMealyModel,
    StatePayload,
    TransitionPayload,
    runStrFAResponse,
    multiRunStrFAResponse> = {
        async createAutomaton (name: string) {
            const response = await mealyApi.createAutomaton(name);
            if (response.status !== 200) {
                throw new Error(`createNewMealy: status ${response.status}`);
            }
            markDirty(response.automaton.id, true);
            return response;
        },

        async deleteAutomaton (automaton_id: number) {
            mealyApi.deleteAutomaton(automaton_id);
        },

        async addState (automatonId: number, payload: StatePayload) {
            if (payload.type !== "MEALY") throw new Error("Ожидался MEALY");
            if (payload.label === undefined) throw new Error("Ожидался MEALY");
            if (payload.x === undefined) throw new Error("Ожидался MEALY");
            if (payload.y === undefined) throw new Error("Ожидался MEALY");
            if (payload.isInitial === undefined) throw new Error("Ожидался MEALY");
            const response = await mealyApi.addState({
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
            await mealyApi.removeState({ automatonId, stateId });
            markDirty(automatonId, true);
        },
        async updateState (automatonId: number, stateId: number, payload: StatePayload) {
            if (payload.type !== "MEALY") throw new Error("Ожидался MEALY");

            const response = await mealyApi.updateState({
                automatonId,
                stateId,
                x: payload.x,
                y: payload.y,
                isInitial: payload.isInitial,
                label: payload.label,
            });
            markDirty(automatonId, true);
            return response;
        },
    };

