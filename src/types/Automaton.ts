import { ComponentType, SVGProps } from "react";
import { AutomatonType } from "./Payloads";

export interface BaseStateModel {
    id: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
}

export interface BaseTransitionModel {
    id: number;
    from: number;
    to: number;
}

export interface StateFAModel extends BaseStateModel {
    isFinal: boolean;
}

export type StateMealyModel = BaseStateModel

export interface TransitionFAModel extends BaseTransitionModel {
    symbol: string;
}

export interface TransitionMealyModel extends BaseTransitionModel {
    symbol: string;
    output: string;
}

export type StateModel = StateFAModel | StateMealyModel;
export type TransitionModel = TransitionFAModel | TransitionMealyModel;

export interface BaseAutomatonModel<TState, TTransition> {
    id: number;
    name: string;
    states: TState[];
    transitions: TTransition[];
}

export type FAAutomatonModel = BaseAutomatonModel<StateFAModel, TransitionFAModel>;
export type MealyAutomatonModel = BaseAutomatonModel<StateMealyModel, TransitionMealyModel>;
export type AutomatonModel = FAAutomatonModel | MealyAutomatonModel;

export interface model {
    title: string;
    type: AutomatonType;
    icon: ComponentType<SVGProps<SVGSVGElement>>;
    description: string;
}