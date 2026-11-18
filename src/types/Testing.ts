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

export type lineTestFA = {
    line: string;
    isFinal: boolean;
    correctSymbols: number;
}
export type lineTestMealy= {
    line: string;
    output: string;
    isComplete: boolean;
}

export type lineTest = lineTestFA | lineTestMealy

export type RunStep = RunStepFA | RunStepMealy

export type Trace<T extends RunStep = RunStep> = {
    steps: T[];
    isFinal: boolean;
}
