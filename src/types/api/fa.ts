import { ApiResponse } from "@/types/api/common.ts";
import { FAAutomatonModel, StateFAModel, TransitionFAModel } from "@/types/Automaton.ts";
import { lineTestFA, RunStepFA, Trace } from "@/types/Testing.ts";

export type CreateFAResponse = ApiResponse<{ automaton: FAAutomatonModel }>;

export type CreateFARequest = {
    name: string;
};

export type DeleteFAResponse = ApiResponse;

export type DeleteFARequest = {
    automatonId: number;
};

export type AddStateFARequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}

export type AddStateFAResponse = ApiResponse<{ state: StateFAModel }>;

export type RemoveStateFARequest = {
    automatonId: number;
    stateId: number;
}

export type RemoveStateFAResponse = ApiResponse;

export type UpdateStateFARequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
    isFinal?: boolean;
}

export type UpdateStateFAResponse = ApiResponse<{ state: StateFAModel }>;

export type AddTransitFARequest = {
    automatonId: number;
    from: number;
    to: number;
    symbols: string[];
}

export type AddTransitFAResponse = ApiResponse<{ transition: TransitionFAModel[] }>;

export type UpdateTransitFARequest = {
    automatonId: number;
    transitionId: number;
    newFrom?: number;
    newTo?: number;
    newSymbol?: string;
}

export type UpdateTransitFAResponse = ApiResponse<{ transition: TransitionFAModel[] }>;

export type RemoveTransitFARequest = {
    automatonId: number;
    transitionId: number;
}

export type RemoveTransitFAResponse = ApiResponse;


export type RunStrFARequest = {
    automatonId: number;
    input: string;
}

export type RunStrFAResponse = ApiResponse<{ traces: Trace<RunStepFA>[] }>;


export type MultiRunStrFARequest = {
    automatonId: number;
    inputs: string[];
}

export type MultiRunStrFAResponse = ApiResponse<{ traces: lineTestFA[] }>;

export type GenerateInputsRequest = {
    automatonId: number;
}

export type GenerateInputsResponse = ApiResponse<{ inputs: string[] }>;