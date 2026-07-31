import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel } from "@/interface/Automaton.ts";

interface createNewNFAResponse {
    status: number;
    message: string;
    automaton: AutomatonModel
}

export const createNewNFA = async (name: string): Promise<createNewNFAResponse> => {
    try {
        const response = await invoke<createNewNFAResponse>("create_new_nfa", { name });

        return response;
    } catch (error) {
        console.error("Ошибка при вызове create_new_nfa:", error);
        throw error;
    }
};