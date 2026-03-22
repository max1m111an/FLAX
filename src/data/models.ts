import { ComponentType, SVGProps } from "react";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";
import ArrowUp from "@/assets/svg/ArrowUp.svg?react";

export interface model{
    id: number;
    type: string;
    icon: ComponentType<SVGProps<SVGSVGElement>>;
    description: string;
}

export const models: model[] = [
    {
        id: 0,
        type: "Конечный автомат",
        icon: CircleDot,
        description: "Моделирование НКА, ДКА",
    },
    {
        id: 1,
        type: "КС-грамматика",
        icon: ArrowUp,
        description: "Работа с магазинными автоматами для КС-языков",
    },
    {
        id: 0,
        type: "Машина Тьюринга",
        icon: Processor,
        description: "Создание полных вычислительных моделей",
    },
];