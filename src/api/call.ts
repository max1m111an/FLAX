import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { ApiCommands } from "@/api/commands.ts";

/**
 * Универсальная функция вызова Tauri-команд с централизованной проверкой статуса ответа.
 * @param command Имя команды на бэкенде.
 * @param args Аргументы команды.
 * @returns Типизированный ответ бэкенда.
 */
export async function call<TCommand extends keyof ApiCommands>(
    command: TCommand,
    args: ApiCommands[TCommand]["req"],
): Promise<ApiCommands[TCommand]["res"]> {
    try {
        const response = await invoke<ApiCommands[TCommand]["res"]>(
            command,
            args as InvokeArgs,
        );

        if (response.status !== 200 && ![ 401, 402 ].includes(response.status)) {
            throw new Error(`Ошибка API: ${response.message || "Неизвестная ошибка"}`);
        }

        return response;
    } catch (error) {
        console.error(`[API Error] ${command}:`, error);
        throw error;
    }
}