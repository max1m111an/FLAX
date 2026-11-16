import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { AutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { mealyApi } from "@/api/MealyAPI.ts";
import { multiRunStrMealyResponse, runStrMealyTracesResponse } from "@/types/api/mealy.ts";
import { MealyStatePayload } from "@/types/Payloads.ts";


// @ts-expect-error: todo: add insufficient Api
export const mealyService: IAutomatonService<
    AutomatonModel,
    StateMealyModel,
    TransitionMealyModel,
    MealyStatePayload,
    runStrMealyTracesResponse,
    multiRunStrMealyResponse
> = {
    async createAutomaton(name: string) {
        const response = await mealyApi.createAutomaton(name);
        if (response.status !== 200) throw new Error(`createNewMealy: status ${response.status}`);
        markDirty(response.automaton.id, true);
        return response;
    },

    async deleteAutomaton(automaton_id: number) {
        await mealyApi.deleteAutomaton(automaton_id);
    },

    async addState(automatonId: number, payload: MealyStatePayload) {
        const response = await mealyApi.addState({
            automatonId,
            label: payload.label!,
            x: payload.x!,
            y: payload.y!,
            isInitial: payload.isInitial!,
        });
        markDirty(automatonId, true);
        return response;
    },

    async removeState(automatonId: number, stateId: number) {
        await mealyApi.removeState({ automatonId, stateId });
        markDirty(automatonId, true);
    },

    async updateState(automatonId: number, stateId: number, payload: MealyStatePayload) {
        const response = await mealyApi.updateState({ automatonId, stateId, ...payload });
        markDirty(automatonId, true);
        return response;
    },

    async addTransition(automatonId: number, from: number, to: number, symbolStr: string) {
        const chunks = symbolStr.trim().split(/\s+/);
        const newTransitions: TransitionMealyModel[] = [];

        for (const chunk of chunks) {
            const parts = chunk.split(/[→\-,;]/);
            const input = parts.length >= 2 ? parts[0] : (chunk.length === 2 ? chunk[0] : undefined);
            const output = parts.length >= 2 ? parts[1] : (chunk.length === 2 ? chunk[1] : undefined);

            if (input && output) {
                const response = await mealyApi.addTransition({ automatonId, from, to, input, output });
                newTransitions.push(...response.transition);
            }
        }

        markDirty(automatonId, true);
        return { status: 200, message: "OK", transition: newTransitions };
    },

    async removeTransition(automatonId: number, transitionId: number) {
        await mealyApi.removeTransit({ automatonId, transitionId });
        markDirty(automatonId, true);
    },

    async runStr(automatonId: number, input: string): Promise<runStrMealyTracesResponse> {
        const response = await mealyApi.runStr({ automatonId, input });
        return {
            status: response.status,
            message: response.message,
            traces: [ { steps: response.steps, isFinal: response.status === 200 } ],
        };
    },

    async runMultipleStr(automatonId: number, inputs: string[]) {
        return await mealyApi.multiRunStr({ automatonId, inputs });
    },
};