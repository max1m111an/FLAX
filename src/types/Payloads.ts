export type AutomatonType = "FA" | "SETTINGS" | "MEALY" | "AT" | "GRAMMAR" | "MOORE";

export interface BaseStatePayload {
    label?: string;
    isInitial?: boolean;
    x?: number;
    y?: number;
}

export interface BaseTransitionPayload {
    from: number;
    to: number;
}

export interface FAStatePayload extends BaseStatePayload {
    type: "FA";
    isFinal?: boolean;
}

export interface MealyStatePayload extends BaseStatePayload {
    type: "MEALY";
}

export type StatePayload = FAStatePayload | MealyStatePayload;

export interface FATransitionPayload extends BaseTransitionPayload {
    type: "FA";
    symbol: string;
}

export interface MealyTransitionPayload extends BaseTransitionPayload {
    type: "MEALY";
    symbol: string;
    outputSymbol: string;
}

export type TransitionPayload = FATransitionPayload | MealyTransitionPayload;