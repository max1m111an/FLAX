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

export type RunStep = RunStepFA | RunStepMealy

export type Trace<T extends RunStep> = {
    steps: T[];
    isFinal: boolean;
}
