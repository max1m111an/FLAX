import { ComponentType, SVGProps } from "react";
import { AutomatonType } from "@/types/Payloads.ts";

export interface AutomatonModel {
    id: number;
    name: string;
    states: StateModel[]
    transitions: TransitionModel[];
}

export interface model{
    title: string;
    type: AutomatonType;
    icon: ComponentType<SVGProps<SVGSVGElement>>;
    description: string;
}

export interface StateModel {
    id: number;
    label: string;
    x: number;
    y: number;
    isInitial: boolean;
    isFinal: boolean;
}

export interface TransitionModel {
    id: number;
    from: number
    to: number;
    symbol: string;
}

export interface TransitionMealyModel extends TransitionModel{
    output: string;
}