export type AutomatonType = "FA" | "SETTINGS" | "MEALY" | "AT" | "GRAMMAR" | "MOORE"

export interface FAStatePayload {
    type: "FA";
    label?: string;
    isInitial?: boolean;
    isFinal?: boolean;
    x?: number;
    y?: number;
}

export interface MealyStatePayload {
    type: "MEALY";
    label?: string;
    isInitial?: boolean;
    x?: number;
    y?: number;
}

export type StatePayload = FAStatePayload | MealyStatePayload;

export interface FATransitionPayload {
    type: "FA";
    from: number
    to: number;
    symbol: string;
}

export interface MealyTransitionPayload {
    type: "MEALY";
    inputSymbol: string;
    outputSymbol: string;
}

export type TransitionPayload = FATransitionPayload | MealyTransitionPayload;