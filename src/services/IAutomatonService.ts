import { ApiResponse } from "@/types/api/common.ts";

export interface IAutomatonService<
    TAutomaton,
    TState,
    TTransition,
    TStatePayload,
    TTransitionPayload,
    TRunResponse,
    TMultiRunResponse
> {
    createAutomaton(name: string): Promise<ApiResponse<{ automaton: TAutomaton }>>;
    deleteAutomaton(id: number): Promise<void>;

    saveAutomaton(automatonId: number, path: string): Promise<void>;
    loadAutomaton(path: string): Promise<ApiResponse<{ automaton: TAutomaton }>>;

    addState(automatonId: number, payload: TStatePayload): Promise<ApiResponse<{ state: TState }>>;
    updateState(automatonId: number, stateId: number, payload: Partial<TStatePayload>): Promise<ApiResponse<{ state: TState }>>;
    removeState(automatonId: number, stateId: number): Promise<void>;

    addTransition(automatonId: number, payload: TTransitionPayload): Promise<ApiResponse<{ transition: TTransition[] }>>;
    removeTransition(automatonId: number, transitionId: number): Promise<void>;

    runStr(automatonId: number, input: string): Promise<TRunResponse>;
    runMultipleStrings(automatonId: number, inputs: string[]): Promise<TMultiRunResponse>;
    generateTestInputs(automatonId: number): Promise<ApiResponse<{ inputs: string[] }>>;
}