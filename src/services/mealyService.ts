import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { AutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { StatePayload, TransitionPayload } from "@/types/Payloads.ts";
import { multiRunStrFAResponse } from "@/types/api/fa.ts";
import { mealyApi } from "@/api/MealyAPI.ts";
import { runStrMealyTracesResponse } from "@/types/api/mealy.ts";

// @ts-expect-error: todo: add insufficient Api
export const mealyService: IAutomatonService<
    AutomatonModel,
    StateMealyModel,
    TransitionMealyModel,
    StatePayload,
    TransitionPayload,
    runStrMealyTracesResponse,
    multiRunStrFAResponse> = {
        async createAutomaton(name: string) {
            const response = await mealyApi.createAutomaton(name);
            if (response.status !== 200) {
                throw new Error(`createNewMealy: status ${response.status}`);
            }
            markDirty(response.automaton.id, true);
            return response;
        },

        async deleteAutomaton(automaton_id: number) {
            await mealyApi.deleteAutomaton(automaton_id);
        },

        async addState(automatonId: number, payload: StatePayload) {
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

        async removeState(automatonId: number, stateId: number) {
            await mealyApi.removeState({ automatonId, stateId });
            markDirty(automatonId, true);
        },

        async updateState(automatonId: number, stateId: number, payload: StatePayload) {
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

        async addTransition(automatonId: number, from: number, to: number, symbolStr: string) {
            const chunks = symbolStr.trim().split(/\s+/);
            const newTransitions = [];
            for (const chunk of chunks) {
                const input = chunk[0];
                const output = chunk[1];

                if (input && output) {
                    const response = await mealyApi.addTransition({
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
            return { transition: newTransitions };
        },

        async removeTransition (automatonId: number, transitionId: number) {
            await mealyApi.removeTransit({ automatonId, transitionId });
            markDirty(automatonId, true);
        },
        async runStr(automatonId: number, input: string): Promise<runStrMealyTracesResponse> {
            const response = await mealyApi.runStr({ automatonId, input });

            return {
                status: response.status,
                message: response.message,
                traces: [ {
                    steps: response.steps,
                    isFinal: response.status === 200,
                } ],
            };
        },

    };

