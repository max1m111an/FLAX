import { ApiResponse } from "@/types/api/common.ts";
import { MealyAutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";
import { LineTestMealy, RunStepMealy, Trace } from "@/types/Testing.ts";

export type CreateMealyResponse = ApiResponse<{ automaton: MealyAutomatonModel }>;
export type CreateMealyRequest = {
    name: string;
};
export type DeleteMealyResponse = ApiResponse;
export type DeleteMealyRequest = {
    automatonId: number;
};
export type AddStateMealyRequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
};

export type AddStateMealyResponse = ApiResponse<{ state: StateMealyModel }>;

export type UpdateStateMealyRequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
};

export type UpdateStateMealyResponse = ApiResponse<{ state: StateMealyModel }>;

export type RemoveStateMealyRequest = {
    automatonId: number;
    stateId: number;
};

export type RemoveStateMealyResponse = ApiResponse;

export type AddTransitMealyRequest = {
    automatonId: number;
    from: number;
    to: number;
    input: string;
    output: string;
};

export type AddTransitMealyResponse = ApiResponse<{ transition: TransitionMealyModel[] }>;

export type UpdateTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
    input?: string;
    output?: string;
    newFrom?: number;
    newTo?: number;
};

export type UpdateTransitMealyResponse = ApiResponse<{ transition: TransitionMealyModel[] }>;

export type RemoveTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
};

export type RemoveTransitMealyResponse = ApiResponse;


export type RunStrMealyRequest = {
    automatonId: number;
    input: string;
}

export type RunStrMealyResponse = ApiResponse<{
    output: string;
    steps: RunStepMealy[];
}>;

export type RunStrMealyTracesResponse = ApiResponse<{
    traces: Trace<RunStepMealy>[];
}>;

export type MultiRunStrMealyRequest = {
    automatonId: number;
    inputs: string[];
}

export type MultiRunStrMealyResponse = ApiResponse<{
    results: LineTestMealy[];
}>;