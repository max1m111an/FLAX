import { ComponentType, SVGProps } from "react";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";
import Mealy from "@/assets/svg/Mealy.svg?react";
import Cat from "@/assets/svg/Cat.svg?react";
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
        id: 2,
        type: "Машина Тьюринга",
        icon: Processor,
        description: "Создание полных вычислительных моделей",
    },
    {
        id: 3,
        type: "Автомат Мили",
        icon: Mealy,
        description: "Моделирование автомата-преобразователя с генерацией сигналов на переходах",
    },
    {
        id: 4,
        type: "Автомат Мура",
        icon: Cat,
        description: "Моделирование автомата-преобразователя с генерацией сигналов в состояниях",
    },
    {
        id: 1,
        type: "КС-грамматика",
        icon: ArrowUp,
        description: "Работа с магазинными автоматами для КС-языков",
    },
];