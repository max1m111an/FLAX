import { ApiResponse } from "@/types/api/common.ts";
import { FAAutomatonModel, StateFAModel, TransitionFAModel } from "@/types/Automaton.ts";

export type createFAResponse = ApiResponse<{ automaton: FAAutomatonModel }>;

export type deleteFAResponse = ApiResponse;

export type addStateFARequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}

export type addStateFAResponse = ApiResponse<{ state: StateFAModel }>;

export type removeStateFARequest = {
    automatonId: number;
    stateId: number;
}

export type removeStateFAResponse = ApiResponse;

export type updateStateFARequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
    isFinal?: boolean;
}

export type updateStateFAResponse = ApiResponse<{ state: StateFAModel }>;

export type addTransitFARequest = {
    automatonId: number;
    from: number;
    to: number;
    symbols: string[];
}

export type addTransitFAResponse = ApiResponse<{ transition: TransitionFAModel[] }>;

export type updateTransitFARequest = {
    automatonId: number;
    transitionId: number;
    newFrom?: number;
    newTo?: number;
    newSymbol?: string;
}

export type updateTransitFAResponse = ApiResponse<{ transition: TransitionFAModel[] }>;

export type removeTransitFARequest = {
    automatonId: number;
    transitionId: number;
}

export type removeTransitFAResponse = ApiResponse;

export type RunStep = {
    from: number;
    symbol: string;
    to: number;
}

export type Trace = {
    steps: RunStep[];
    isFinal: boolean;
}

export type runStrFARequest = {
    automatonId: number;
    input: string;
}

export type runStrFAResponse = ApiResponse<{ traces: Trace[] }>;

export type lineTest = {
    line: string;
    isFinal: boolean;
    correctSymbols: number;
}

export type multiRunStrFARequest = {
    automatonId: number;
    inputs: string[];
}

export type multiRunStrFAResponse = ApiResponse<{ traces: lineTest[] }>;

export type generateInputsRequest = {
    automatonId: number;
}

export type generateInputsResponse = ApiResponse<{ inputs: string[] }>;