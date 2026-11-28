export type RunStepFA = {
    from: number;
    symbol: string;
    to: number;
}

export type RunStepMealy = {
    from: number;
    symbol: string;
    to: number;
    output: string;
}

export type LineTestFA = {
    line: string;
    isFinal: boolean;
    correctSymbols: number;
}
export type LineTestMealy = {
    line: string;
    output: string;
    isComplete: boolean;
}

export type LineTest = LineTestFA | LineTestMealy

export type RunStep = RunStepFA | RunStepMealy

export type Trace<T extends RunStep = RunStep> = {
    steps: T[];
    isFinal: boolean;
}
