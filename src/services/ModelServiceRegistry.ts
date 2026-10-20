import { IAutomatonService } from "./IAutomatonService";
import { faService } from "./faService";
import { AutomatonType } from "@/types/Payloads.ts";

export type AnyAutomatonService = IAutomatonService<any, any, any, any, any, any, any>;

const services: Partial<Record<AutomatonType, AnyAutomatonService>> = {
    "FA": faService,
};

const isSupportedType = (type: string): type is AutomatonType => {
    return type === "FA" || type === "MEALY";
};

export const modelServiceRegistry = {
    getService(type: string): AnyAutomatonService {
        const normalizedType = type.toUpperCase();

        if (!isSupportedType(normalizedType) || !services[normalizedType]) {
            throw new Error(`[ModelServiceRegistry] Сервис для типа автомата "${type}" не найден или не поддерживается.`);
        }

        return services[normalizedType]!;
    },
};