import { markDirty } from "@/services/dirtyState.ts";
import { loadJFFRequest, loadJFFResponse, saveJFFRequest, saveJFFResponse } from "@/types/api/jff.ts";
import { jffAPI } from "@/api/jffAPI.ts";


export const jffService = {
    async loadJff(params: loadJFFRequest): Promise<loadJFFResponse> {
        const response = await jffAPI.loadJFF(params);
        if (response.status !== 200) {
            throw new Error(`loadJff: status ${response.status}`);
        }
        return response;
    },
    async saveJff (params: saveJFFRequest): Promise<saveJFFResponse> {
        const response = await jffAPI.saveJFF(params);
        if (response.status !== 200) {
            throw new Error(`saveJff: status ${response.status}`);
        }
        markDirty(params.automatonId, false);
        return response;
    },
};