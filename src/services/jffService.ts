import { loadJFF, saveJFF } from "@/api/jffAPI.ts";
import type { loadJFFRequest, loadJFFResponse, saveJFFRequest, saveJFFResponse } from "@/api/jffAPI.ts";
import { markDirty } from "@/services/dirtyState.ts";

export type { loadJFFRequest, loadJFFResponse, saveJFFRequest, saveJFFResponse } from "@/api/jffAPI.ts";

export const loadJff = async (params: loadJFFRequest): Promise<loadJFFResponse> => {
    const response = await loadJFF(params);
    if (response.status !== 200) {
        throw new Error(`loadJff: status ${response.status}`);
    }
    return response;
};

export const saveJff = async (params: saveJFFRequest): Promise<saveJFFResponse> => {
    const response = await saveJFF(params);
    if (response.status !== 200) {
        throw new Error(`saveJff: status ${response.status}`);
    }
    markDirty(params.automatonId, false);
    return response;
};