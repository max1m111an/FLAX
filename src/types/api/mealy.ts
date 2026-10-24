import { ApiResponse } from "@/types/api/common.ts";
import { MealyAutomatonModel, StateMealyModel, TransitionMealyModel } from "@/types/Automaton.ts";

export type createMealyResponse = ApiResponse<{ automaton: MealyAutomatonModel }>;

export type deleteMealyResponse = ApiResponse;

export type addStateMealyRequest = {
    automatonId: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
};

export type addStateMealyResponse = ApiResponse<{ state: StateMealyModel }>;

export type updateStateMealyRequest = {
    automatonId: number;
    stateId: number;
    label?: string;
    x?: number;
    y?: number;
    isInitial?: boolean;
};

export type updateStateMealyResponse = ApiResponse<{ state: StateMealyModel }>;

export type removeStateMealyRequest = {
    automatonId: number;
    stateId: number;
};

export type removeStateMealyResponse = ApiResponse;

export type addTransitMealyRequest = {
    automatonId: number;
    from: number;
    to: number;
    input: string;
    output: string;
};

export type addTransitMealyResponse = ApiResponse<{ transition: TransitionMealyModel[] }>;

export type updateTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
    input?: string;
    output?: string;
    newFrom?: number;
    newTo?: number;
};

export type updateTransitMealyResponse = ApiResponse<{ transition: TransitionMealyModel[] }>;

export type removeTransitMealyRequest = {
    automatonId: number;
    transitionId: number;
};

export type removeTransitMealyResponse = ApiResponse;