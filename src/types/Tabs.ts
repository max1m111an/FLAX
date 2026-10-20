import { AutomatonModel } from "@/types/Automaton.ts";
import { ComponentType, SVGProps } from "react";
import { AutomatonType } from "@/types/Payloads.ts";
import { Trace } from "@/types/api/fa.ts";


export interface TraceHighlight {
    id: number;
    status?: "success" | "error";
}

export interface BaseTab {
    id: number;
    title: string;
    type: AutomatonType;
    isSaved: boolean;
    savedPath: string | null;
    icon: ComponentType<SVGProps<SVGSVGElement>>;
}

export interface FATab extends BaseTab{
    automaton: AutomatonModel;
    activeControl: ActiveControl;
    activePanel: string | null;
    selectedState: TraceHighlight[] | null;
    selectedTransition: TraceHighlight[] | null;
    selectedNodeId: number | null;
    testMode: string;
    testInput: string;
    pendingTestLine: string | null;
    pendingTraces: Trace[] | null;
}

export interface SettingsTab extends BaseTab {
    type: "SETTINGS";
}

export type AppTab = FATab | SettingsTab;
export type ActiveControl = "move" | "edge" | "node" | "trashcan";