import { markDirty } from "@/services/dirtyState.ts";
import { createNewMealy, createNewMealyResponse, deleteMealyApi, deleteMealyResponse } from "@/api/MealyAPI.ts";


export const createMealy = async (name: string): Promise<createNewMealyResponse> => {
    const response = await createNewMealy(name);
    if (response.status !== 200) {
        throw new Error(`createNewMealy: status ${response.status}`);
    }
    markDirty(response.automaton.id, true);
    return response;
};

export const deleteMealyService = async (automaton_id: number): Promise<deleteMealyResponse> => {
    const response = await deleteMealyApi(automaton_id);
    if (response.status !== 200) {
        throw new Error(`deleteFA: status ${response.status}`);
    }
    return response;
};

