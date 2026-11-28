import { markDirty } from "@/services/dirtyState.ts";
import { LoadJFFRequest, LoadJFFResponse, SaveJFFRequest, SaveJFFResponse } from "@/types/api/jff.ts";
import { jffAPI } from "@/api/jffAPI.ts";


export const jffService = {
    async loadJff(params: LoadJFFRequest): Promise<LoadJFFResponse> {
        const response = await jffAPI.loadJFF(params);
        if (response.status !== 200) {
            throw new Error(`loadJff: status ${response.status}`);
        }
        return response;
    },
    async saveJff (params: SaveJFFRequest): Promise<SaveJFFResponse> {
        const response = await jffAPI.saveJFF(params);
        if (response.status !== 200) {
            throw new Error(`saveJff: status ${response.status}`);
        }
        markDirty(params.automatonId, false);
        return response;
    },
};