import Plus from "@/assets/svg/Plus.svg?react";
import ChevronRight from "@/assets/svg/ChevronRight.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import Cancel from "@/assets/svg/Cancel.svg?react";
import ChevronDown from "@/assets/svg/ChevronDown.svg?react";
import Save from "@/assets/svg/Save.svg?react";
import { useState } from "react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Button } from "@/components/ui/Button/Button.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { IconButton } from "@/components/ui/IconButton/IconButton.tsx";
import styles from "./ModelProperties.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { addTransitionNFA, removeTransitNFA, updateTransitNFA } from "@/api/nfaAPI.ts";

export default function EdgeProperties() {
    const [ isOpenId, setIsOpenId ] = useState<number[]>([]);
    const [ editingValues, setEditingValues ] = useState<Record<number, string>>({});
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const [ isAdding, setIsAdding ] = useState(false);
    const [ newEdgeTo, setNewEdgeTo ] = useState<number | null>(null);
    const [ newEdgeSymbols, setNewEdgeSymbols ] = useState<string>("");

    if (!currentTab) return null;
    const selectedState = currentTab.selectedState?.[0]?.id ?? null;
    if (selectedState == null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }

    const outTransitions = currentTab.automaton.transitions.filter((t) => t.from === selectedState);
    const grouped = outTransitions.reduce((acc, t) => {
        if (!acc[t.to]) acc[t.to] = { to: t.to, ids: [], symbols: [] };
        acc[t.to].ids.push(t.id);
        acc[t.to].symbols.push(t.symbol);
        return acc;
    }, {} as Record<number, { to: number; ids: number[]; symbols: string[] }>);

    const currentEdges = Object.values(grouped);

    const handleEndStateChange = async (oldTo: number, newTo: number) => {
        if (oldTo === newTo) return;
        const group = grouped[oldTo];
        const updatedTransitions = [];

        for (const id of group.ids) {
            const res = await updateTransitNFA({
                automatonId: currentTab.id,
                transitionId: id,
                new_to: newTo,
            });
            if (res.status === 200) {
                updatedTransitions.push(...res.transition);
            }
        }

        const newTabData = {
            ...currentTab,
            automaton: {
                ...currentTab.automaton,
                transitions: currentTab.automaton.transitions
                    .filter((t) => !group.ids.includes(t.id))
                    .concat(updatedTransitions),
            },
        };

        updateTab(newTabData);
    };

    function validateAndParse(value: string): { valid: boolean; symbols: string[] } {
        const raw = value.split(",").map((s) => s.trim());
        const filtered = raw.filter((s) => s !== "");
        let unique = [ ...new Set(filtered) ];

        if (unique.length > 1 && unique.includes("λ")) {
            unique = unique.filter((s) => s !== "λ");
        }

        return {
            valid: true,
            symbols: unique.length > 0 ? unique : [ "λ" ],
        };
    }

    const handleStateChange = (to: number, value: string) => {
        setEditingValues((prev) => ({ ...prev, [to]: value }));
    };

    const handleSave = async (to: number) => {
        const value = editingValues[to] ?? "";
        const result = validateAndParse(value);

        if (!result.valid) return;

        const group = grouped[to];

        for (const id of group.ids) {
            await removeTransitNFA({ automatonId: currentTab.id, transitionId: id });
        }

        const res = await addTransitionNFA({
            automatonId: currentTab.id,
            from: selectedState,
            to: to,
            symbols: result.symbols,
        });

        if (res.status === 200) {
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: currentTab.automaton.transitions
                        .filter((t) => !group.ids.includes(t.id))
                        .concat(res.transition),
                },
            };

            updateTab(newTabData);

            setEditingValues((prev) => {
                const copy = { ...prev };
                delete copy[to];
                return copy;
            });
        }
    };

    const handleDeleteGroup = async (to: number, e: React.MouseEvent) => {
        e.stopPropagation();
        const group = grouped[to];
        for (const id of group.ids) {
            await removeTransitNFA({ automatonId: currentTab.id, transitionId: id });
        }

        const newTabData = {
            ...currentTab,
            automaton: {
                ...currentTab.automaton,
                transitions: currentTab.automaton.transitions.filter((t) => !group.ids.includes(t.id)),
            },
        };

        updateTab(newTabData);
    };

    const confirmAddEdge = async () => {
        if (newEdgeTo === null) return;

        const result = validateAndParse(newEdgeSymbols);
        if (!result.valid) return;

        const res = await addTransitionNFA({
            automatonId: currentTab.id,
            from: selectedState,
            to: newEdgeTo,
            symbols: result.symbols,
        });

        if (res.status === 200) {
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: [ ...currentTab.automaton.transitions, ...res.transition ],
                },
            };

            updateTab(newTabData);
            setIsAdding(false);
            setNewEdgeTo(null);
            setNewEdgeSymbols("");
        }
    };

    return (
        <>
            <Typography variant="pretitle">
                Переходы из {currentTab.automaton.states.find((node) => node.id === selectedState)?.label}:
            </Typography>
            {currentEdges.length > 0 &&
                currentEdges.map((edgeGroup) => (
                    <div key={ edgeGroup.to } className={ styles.propFullCardEdgeWrapper }>
                        <div
                            className={ styles.propCardEdgeWrapper }
                            onClick={ () => {
                                if (isOpenId.includes(edgeGroup.to)) {
                                    setIsOpenId(isOpenId.filter((id) => id !== edgeGroup.to));
                                } else {
                                    setIsOpenId([ ...isOpenId, edgeGroup.to ]);
                                }
                            } }
                        >
                            {!isOpenId.includes(edgeGroup.to) ? <ChevronRight className ={ styles.icon } /> : <ChevronDown className ={ styles.icon } />}
                            <div className={ styles.nodeEdgeWrapper }>
                                <p className={ styles.nodeToNodeTitle }>
                                    {currentTab.automaton.states.find((node) => node.id === selectedState)?.label}
                                    <ArrowRight />
                                    {currentTab.automaton.states.find((node) => node.id === edgeGroup.to)?.label}
                                </p>
                                <p className={ styles.stateDescription }>{edgeGroup.symbols.join(", ")}</p>
                            </div>
                            <IconButton variant="cancel" onClick={ (e) => handleDeleteGroup(edgeGroup.to, e) }>
                                <Cancel />
                            </IconButton>
                        </div>
                        {isOpenId.includes(edgeGroup.to) && (
                            <div className={ styles.propOpenCardWrapper }>
                                <p className={ styles.pretitle2 }>В состояние</p>
                                <DropdownMenu.Root>
                                    <DropdownMenu.Trigger className={ styles.dropdownInput }>
                                        {currentTab.automaton.states.find((node) => node.id === edgeGroup.to)?.label}
                                    </DropdownMenu.Trigger>

                                    <DropdownMenu.Content
                                        className={ styles.nodesDropdown }
                                        style={ { width: "var(--radix-dropdown-menu-trigger-width)" } }
                                    >
                                        <DropdownMenu.Group>
                                            {currentTab.automaton.states.map((node) => (
                                                <DropdownMenu.Item
                                                    key={ node.id }
                                                    className={ styles.nodeItem }
                                                    onClick={ () => handleEndStateChange(edgeGroup.to, node.id) }
                                                >
                                                    {node.label}
                                                </DropdownMenu.Item>
                                            ))}
                                        </DropdownMenu.Group>
                                    </DropdownMenu.Content>
                                </DropdownMenu.Root>
                                <p className={ styles.pretitle2 }>Символы перехода</p>
                                <div className={ styles.edgeSaveWrapper }>
                                    <Textfield
                                        value={ editingValues[edgeGroup.to] ?? edgeGroup.symbols.join(", ") }
                                        onChange={ (e) => handleStateChange(edgeGroup.to, e.target.value) }
                                        onKeyDown={ (e) => {
                                            if (e.key === "Enter") {
                                                handleSave(edgeGroup.to);
                                            }
                                        } }
                                    />
                                    <Button
                                        variant="main"
                                        square
                                        onClick={ () => handleSave(edgeGroup.to) }
                                        disabled={
                                            (editingValues[edgeGroup.to] ?? edgeGroup.symbols.join(", ")) ===
                                            edgeGroup.symbols.join(", ")
                                        }
                                    >
                                        <Save />
                                    </Button>
                                </div>
                            </div>
                        )}
                    </div>
                ))}

            {!isAdding ? (
                <Button variant="main" onClick={ () => setIsAdding(true) }>
                    <Plus />
                </Button>
            ) : (
                <div className={ styles.propFullCardEdgeWrapper }>
                    <div className={ styles.propCardEdgeWrapper } style={ { cursor: "default" } }>
                        <div className={ styles.nodeEdgeWrapper }>
                            <p className={ styles.nodeToNodeTitle }>
                                {currentTab.automaton.states.find((node) => node.id === selectedState)?.label}
                                <ArrowRight className ={ styles.icon } />
                                {newEdgeTo !== null ? currentTab.automaton.states.find((node) => node.id === newEdgeTo)?.label : "?"}
                            </p>
                        </div>
                        <IconButton variant="cancel" onClick={ () => setIsAdding(false) }>
                            <Cancel />
                        </IconButton>
                    </div>
                    <div className={ styles.propOpenCardWrapper }>
                        <p className={ styles.pretitle2 }>В состояние</p>
                        <DropdownMenu.Root>
                            <DropdownMenu.Trigger className={ styles.dropdownInput }>
                                {newEdgeTo !== null
                                    ? currentTab.automaton.states.find((node) => node.id === newEdgeTo)?.label
                                    : "Выберите вершину..."}
                            </DropdownMenu.Trigger>

                            <DropdownMenu.Content
                                className={ styles.nodesDropdown }
                                style={ { width: "var(--radix-dropdown-menu-trigger-width)" } }
                            >
                                <DropdownMenu.Group>
                                    {currentTab.automaton.states.map((node) => (
                                        <DropdownMenu.Item
                                            key={ node.id }
                                            className={ styles.nodeItem }
                                            onClick={ () => setNewEdgeTo(node.id) }
                                        >
                                            {node.label}
                                        </DropdownMenu.Item>
                                    ))}
                                </DropdownMenu.Group>
                            </DropdownMenu.Content>
                        </DropdownMenu.Root>
                        <p className={ styles.pretitle2 }>Символы перехода</p>
                        <div className={ styles.edgeSaveWrapper }>
                            <Textfield
                                value={ newEdgeSymbols }
                                onChange={ (e) => setNewEdgeSymbols(e.target.value) }
                                onKeyDown={ (e) => {
                                    if (e.key === "Enter") {
                                        confirmAddEdge();
                                    }
                                } }
                            />
                            <Button
                                variant="main"
                                square
                                onClick={ confirmAddEdge }
                                disabled={ newEdgeTo === null }
                            >
                                <Plus />
                            </Button>
                        </div>
                    </div>
                </div>
            )}
        </>
    );
}
