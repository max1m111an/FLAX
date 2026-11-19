import {
    AddStateMealyRequest,
    AddStateMealyResponse,
    AddTransitMealyRequest,
    AddTransitMealyResponse, CreateMealyRequest,
    CreateMealyResponse, DeleteMealyRequest,
    DeleteMealyResponse,
    MultiRunStrMealyRequest,
    MultiRunStrMealyResponse,
    RemoveStateMealyRequest,
    RemoveStateMealyResponse,
    RemoveTransitMealyRequest,
    RemoveTransitMealyResponse,
    RunStrMealyRequest,
    RunStrMealyResponse,
    UpdateStateMealyRequest,
    UpdateStateMealyResponse,
    UpdateTransitMealyRequest,
    UpdateTransitMealyResponse,
} from "@/types/api/mealy.ts";

export interface MealyCommands {
    "mealy_create_new": { req: CreateMealyRequest; res: CreateMealyResponse };
    "mealy_remove_automaton": { req: DeleteMealyRequest; res: DeleteMealyResponse };
    "mealy_add_state": { req: AddStateMealyRequest; res: AddStateMealyResponse };
    "mealy_update_state": { req: UpdateStateMealyRequest; res: UpdateStateMealyResponse };
    "mealy_remove_state": { req: RemoveStateMealyRequest; res: RemoveStateMealyResponse };
    "mealy_add_transition": { req: AddTransitMealyRequest; res: AddTransitMealyResponse };
    "mealy_update_transition": { req: UpdateTransitMealyRequest; res: UpdateTransitMealyResponse };
    "mealy_remove_transition": { req: RemoveTransitMealyRequest; res: RemoveTransitMealyResponse };
    "mealy_run_str": { req: RunStrMealyRequest; res: RunStrMealyResponse };
    "mealy_multi_run": { req: MultiRunStrMealyRequest; res: MultiRunStrMealyResponse };
}