import {
    AddStateFARequest,
    AddStateFAResponse,
    AddTransitFARequest,
    AddTransitFAResponse, CreateFARequest,
    CreateFAResponse, DeleteFARequest,
    DeleteFAResponse,
    GenerateInputsRequest,
    GenerateInputsResponse,
    MultiRunStrFARequest,
    MultiRunStrFAResponse,
    RemoveStateFARequest,
    RemoveStateFAResponse,
    RemoveTransitFARequest,
    RemoveTransitFAResponse,
    RunStrFARequest,
    RunStrFAResponse,
    UpdateStateFARequest,
    UpdateStateFAResponse,
    UpdateTransitFARequest,
    UpdateTransitFAResponse,
} from "@/types/api/fa.ts";

export interface FaCommands {
    "fa_create_new": { req: CreateFARequest; res: CreateFAResponse };
    "fa_remove_automaton": { req: DeleteFARequest; res: DeleteFAResponse };
    "fa_add_state": { req: AddStateFARequest; res: AddStateFAResponse };
    "fa_remove_state": { req: RemoveStateFARequest; res: RemoveStateFAResponse };
    "fa_update_state": { req: UpdateStateFARequest; res: UpdateStateFAResponse };
    "fa_add_transition": { req: AddTransitFARequest; res: AddTransitFAResponse };
    "fa_update_transition": { req: UpdateTransitFARequest; res: UpdateTransitFAResponse };
    "fa_remove_transition": { req: RemoveTransitFARequest; res: RemoveTransitFAResponse };
    "fa_run_str": { req: RunStrFARequest; res: RunStrFAResponse };
    "fa_multi_run_str": { req: MultiRunStrFARequest; res: MultiRunStrFAResponse };
    "fa_generate_inputs": { req: GenerateInputsRequest; res: GenerateInputsResponse };
}