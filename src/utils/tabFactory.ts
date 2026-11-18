import { AutomatonModel } from "@/types/Automaton";
import { AppTab } from "@/types/Tabs";
import { ComponentType, SVGProps } from "react";
import { AutomatonType } from "@/types/Payloads.ts";

export const buildTab = (
    automaton: AutomatonModel,
    icon: ComponentType<SVGProps<SVGSVGElement>>,
    type: AutomatonType,
    isSaved: boolean = false,
    savedPath: string | null = null,
): AppTab => {
    return {
        id: automaton.id,
        type,
        title: automaton.name,
        icon,
        automaton,

        activeControl: "move",
        activePanel: null,
        selectedState: null,
        selectedTransition: null,
        selectedNodeId: null,
        testMode: "solo",
        testInput: "",
        pendingTestLine: null,
        pendingTraces: null,

        isSaved,
        savedPath,
    };
};