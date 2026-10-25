import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { StatePayload } from "@/types/Payloads.ts";
import { faApi } from "@/api/faAPI.ts";
import { AutomatonModel, StateModel, TransitionModel } from "@/types/Automaton.ts";
import { multiRunStrFAResponse, runStrFAResponse } from "@/types/api/fa.ts";
import { jffAPI } from "@/api/jffAPI.ts";


export const faService: IAutomatonService<
    AutomatonModel,
    StateModel,
    TransitionModel,
    StatePayload,
    runStrFAResponse,
    multiRunStrFAResponse
> = {
    createAutomaton: async (name: string) => {
        const response = await faApi.createAutomaton(name);
        markDirty(response.automaton.id, true);
        return response;
    },
    deleteAutomaton: async (id: number) => {
        await faApi.deleteAutomaton(id);
    },

    saveAutomaton: async (automatonId: number, path: string) => {
        await jffAPI.saveJFF({ automatonId, path });
    },

    loadAutomaton: async (path: string) => {
        return await jffAPI.loadJFF({ path });
    },

    addState: async (automatonId: number, payload: StatePayload) => {
        if (payload.type !== "FA") throw new Error("Ожидался FA");
        if (payload.label === undefined) throw new Error("Ожидался FA");
        if (payload.x === undefined) throw new Error("Ожидался FA");
        if (payload.y === undefined) throw new Error("Ожидался FA");
        if (payload.isInitial === undefined) throw new Error("Ожидался FA");
        if (payload.isFinal === undefined) throw new Error("Ожидался FA");
        const response = await faApi.addState({
            automatonId,
            label: payload.label,
            x: payload.x,
            y: payload.y,
            isInitial: payload.isInitial,
            isFinal: payload.isFinal,
        });
        markDirty(automatonId, true);

        return response;
    },

    updateState: async (automatonId: number, stateId: number, payload: StatePayload) => {
        if (payload.type !== "FA") throw new Error("Ожидался FA");

        const response = await faApi.updateState({
            automatonId,
            stateId,
            x: payload.x,
            y: payload.y,
            isInitial: payload.isInitial,
            isFinal: payload.isFinal,
            label: payload.label,
        });
        markDirty(automatonId, true);
        return response;
    },

    removeState: async (automatonId: number, stateId: number) => {
        await faApi.removeState({ automatonId, stateId });
        markDirty(automatonId, true);
    },

    addTransition: async (automatonId: number, from: number, to: number, symbolStr: string) => {
        let symbols = symbolStr.split(",").map((s) => s.trim()).filter((s) => s !== "");
        if (symbols.length === 0) {
            symbols = [ "$" ];
        }
        const response = await faApi.addTransition({
            automatonId,
            from,
            to,
            symbols,
        });
        markDirty(automatonId, true);
        return response;
    },

    removeTransition: async (automatonId: number, transitionId: number) => {
        await faApi.removeTransit({ automatonId, transitionId });
        markDirty(automatonId, true);
    },

    runStr: async (automatonId: number, input: string) => {
        return await faApi.runStr({ automatonId, input });
    },
    runMultipleStrings: async (automatonId: number, inputs: string[]) => {
        return await faApi.multiRunStr({ automatonId, inputs });
    },
    generateTestInputs: async (automatonId: number) => {
        return await faApi.generateInputs({ automatonId });
    },

};