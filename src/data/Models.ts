import CircleDot from "@/assets/svg/CircleDot.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";
import Mealy from "@/assets/svg/Mealy.svg?react";
import Cat from "@/assets/svg/Cat.svg?react";
import ArrowUp from "@/assets/svg/ArrowUp.svg?react";
import { model } from "@/types/Automaton.ts";


export const models: model[] = [
    {
        type: "FA",
        title: "Конечный автомат",
        icon: CircleDot,
        description: "Моделирование НКА, ДКА",
    },
    {
        type: "AT",
        title: "Машина Тьюринга",
        icon: Processor,
        description: "Создание полных вычислительных моделей",
    },
    {
        type: "MEALY",
        title: "Автомат Мили",
        icon: Mealy,
        description: "Моделирование автомата-преобразователя с генерацией сигналов на переходах",
    },
    {
        type: "MOORE",
        title: "Автомат Мура",
        icon: Cat,
        description: "Моделирование автомата-преобразователя с генерацией сигналов в состояниях",
    },
    {
        type: "GRAMMAR",
        title: "КС-грамматика",
        icon: ArrowUp,
        description: "Работа с магазинными автоматами для КС-языков",
    },
];