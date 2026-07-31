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
    is_initial: boolean;
    is_final: boolean;
}
export interface TransitionModel {
    from: number;
    to: number;
    symbol: string;
    label?: string;
}