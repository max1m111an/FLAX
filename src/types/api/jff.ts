import { ApiResponse } from "@/types/api/common.ts";
import { AutomatonModel } from "@/types/Automaton.ts";

export type loadJFFRequest = {
    path: string;
}
export type loadJFFResponse = ApiResponse<{
    data: AutomatonModel,
    isAutomaton: boolean,
}>;

export type saveJFFRequest = {
    automatonId: number;
    path: string;
}

export type saveJFFResponse = ApiResponse