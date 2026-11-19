import { call } from "@/api/call.ts";
import { markDirty } from "@/services/dirtyState.ts";
import { IAutomatonService } from "@/services/IAutomatonService.ts";
import { StatePayload } from "@/types/Payloads.ts";
import { AutomatonModel, StateModel, TransitionModel } from "@/types/Automaton.ts";
import { MultiRunStrFAResponse, RunStrFAResponse } from "@/types/api/fa.ts";

/**
 * Сервис для работы с конечными автоматами (FA).
 * Инкапсулирует бизнес-логику, валидацию параметров и взаимодействие с бэкендом.
 */
export const faService: IAutomatonService<
    AutomatonModel,
    StateModel,
    TransitionModel,
    StatePayload,
    RunStrFAResponse,
    MultiRunStrFAResponse
> = {
    async createAutomaton (name: string) {
        const response = await call("fa_create_new", { name });
        markDirty(response.automaton.id, true);
        return response;
    },

    async deleteAutomaton (id: number) {
        await call("fa_remove_automaton", { automatonId: id });
    },

    async addState (automatonId: number, payload: StatePayload) {
        if (payload.type !== "FA") {
            throw new Error("Ожидался тип payload 'FA'");
        }
        if (
            payload.label === undefined ||
            payload.x === undefined ||
            payload.y === undefined ||
            payload.isInitial === undefined ||
            payload.isFinal === undefined
        ) {
            throw new Error("Не заполнены обязательные поля для создания состояния FA");
        }

        const response = await call("fa_add_state", {
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

    async updateState (automatonId: number, stateId: number, payload: StatePayload) {
        if (payload.type !== "FA") {
            throw new Error("Ожидался тип payload 'FA'");
        }

        const response = await call("fa_update_state", {
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

    async removeState (automatonId: number, stateId: number) {
        await call("fa_remove_state", { automatonId, stateId });
        markDirty(automatonId, true);
    },

    async addTransition (automatonId: number, from: number, to: number, symbolStr: string) {
        let symbols = symbolStr
            .split("")
            .map((s) => s.trim())
            .filter((s) => s !== "");

        if (symbols.length === 0) {
            symbols = [ "$" ];
        }

        const response = await call("fa_add_transition", {
            automatonId,
            from,
            to,
            symbols,
        });
        markDirty(automatonId, true);

        return response;
    },

    async removeTransition (automatonId: number, transitionId: number) {
        await call("fa_remove_transition", { automatonId, transitionId });
        markDirty(automatonId, true);
    },

    async runStr (automatonId: number, input: string) {
        return await call("fa_run_str", { automatonId, input });
    },

    async runMultipleStr (automatonId: number, inputs: string[]) {
        return await call("fa_multi_run_str", { automatonId, inputs });
    },

    async generateTestInputs (automatonId: number) {
        return await call("fa_generate_inputs", { automatonId });
    },
};