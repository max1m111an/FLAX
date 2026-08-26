export interface AutomatonModel {
    id: number;
    name: string;
    kind: string;
    states: StateModel[]
    transitions: TransitionModel[];
    alphabet: string[];
}
export interface StateModel {
    id: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}

export interface TransitionModel {
    id: number;
    from: number
    to: number;
    symbol: string;
}