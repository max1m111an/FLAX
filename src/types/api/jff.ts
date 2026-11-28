import { ApiResponse } from "@/types/api/common.ts";
import { AutomatonModel } from "@/types/Automaton.ts";

export type LoadJFFRequest = {
    path: string;
}
export type LoadJFFResponse = ApiResponse<{
    data: AutomatonModel,
    isAutomaton: boolean,
}>;

export type SaveJFFRequest = {
    automatonId: number;
    path: string;
}

export type SaveJFFResponse = ApiResponse