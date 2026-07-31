import Plus from "@/assets/svg/Plus.svg?react";
import ChevronRight from "@/assets/svg/ChevronRight.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import Cancel from "@/assets/svg/Cancel.svg?react";
import ChevronDown from "@/assets/svg/ChevronDown.svg?react";
import Save from "@/assets/svg/Save.svg?react";
import { useState } from "react";
import { EdgeState, useControl } from "@/context/ControlContext.tsx";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Button } from "@/components/ui/Button/Button.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { IconButton } from "@/components/ui/IconButton/IconButton.tsx";
import styles from "./ModelProperties.module.scss";


export default function EdgeProperties() {
    const [ isOpenId, setIsOpenId ] = useState<number[]>([]);
    const { selectedNode, nodes, edges, setEdges } = useControl();
    const [ editingValues, setEditingValues ] = useState<Record<number, string>>({});

    const currentEdges = edges.filter((edge) => edge.idStartNode === selectedNode);


    if (selectedNode === null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }
    const handleEndStateChange = (idEdge: number, idNode: number) => {
        setEdges((prev) => prev.map((edge) =>
            edge.id === idEdge
                ? { ...edge, idEndNode: idNode }
                : edge,
        ));
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
    const handleStateChange = (idEdge: number, value: string) => {
        setEditingValues((prev) => ({ ...prev, [idEdge]: value }));
    };
    const handleSave = (id: number) => {
        const value = editingValues[id] ?? "";

        const result = validateAndParse(value);

        if (!result.valid) return;

        setEdges((prev) =>
            prev.map((edge) =>
                edge.id === id
                    ? { ...edge, state: result.symbols }
                    : edge,
            ),
        );

        setEditingValues((prev) => {
            const copy = { ...prev };
            delete copy[id];
            return copy;
        });
    };
    return (
        <>
            <Typography variant="pretitle">Переходы из {nodes.find((node) => node.id === selectedNode)?.name}:</Typography>
            {currentEdges.length > 0 && (
                currentEdges.map((edge: EdgeState) =>
                    (
                        <div key={ edge.id } className={ styles.propFullCardEdgeWrapper }>
                            <div
                                className={ styles.propCardEdgeWrapper }
                                onClick={ () => {
                                    if (isOpenId.includes(edge.id)) {
                                        setIsOpenId(isOpenId.filter((id) => id !== edge.id));
                                    } else {
                                        setIsOpenId([ ...isOpenId, edge.id ]);
                                    }
                                } }
                            >
                                {!isOpenId.includes(edge.id) ? (
                                    <ChevronRight />
                                ) : (
                                    <ChevronDown />
                                )}
                                <div className={ styles.nodeEdgeWrapper }>
                                    <p className={ styles.nodeToNodeTitle }>
                                        {nodes.find((node) => node.id === selectedNode)?.name}
                                        <ArrowRight />
                                        {nodes.find((node) => node.id === edge.idEndNode)?.name}
                                    </p>
                                    <p className={ styles.stateDescription }>
                                        {edge.state?.join(", ")}
                                    </p>
                                </div>
                                <IconButton variant="cancel" onClick={ (e) => { e.stopPropagation(); } }>
                                    <Cancel />
                                </IconButton>
                            </div>
                            {!isOpenId.includes(edge.id) && (
                                <div className={ styles.propOpenCardWrapper }>
                                    <p className={ styles.pretitle2 }>В состояние</p>
                                    <DropdownMenu.Root>
                                        <DropdownMenu.Trigger className={ styles.dropdownInput }>
                                            {nodes.find((node) => node.id === edge.idEndNode)?.name}
                                        </DropdownMenu.Trigger>

                                        <DropdownMenu.Content className={ styles.nodesDropdown }
                                            style={ { width: "var(--radix-dropdown-menu-trigger-width)" } }>
                                            <DropdownMenu.Group>
                                                {nodes.map((node) => {
                                                    return (
                                                        <DropdownMenu.Item key={ node.id } className={ styles.nodeItem } onClick={ () => handleEndStateChange(edge.id, node.id) }>
                                                            {node.name}
                                                        </DropdownMenu.Item>
                                                    );
                                                })}
                                            </DropdownMenu.Group>
                                        </DropdownMenu.Content>
                                    </DropdownMenu.Root>
                                    <p className={ styles.pretitle2 }>Символы перехода</p>
                                    <div className={ styles.edgeSaveWrapper }>
                                        <Textfield
                                            value={
                                                editingValues[edge.id] ??
                                                edge.state?.join(", ") ??
                                                ""
                                            }
                                            onChange={ (e) => handleStateChange(edge.id, e.target.value) }
                                            onKeyDown={ (e) => {
                                                if (e.key === "Enter") {
                                                    handleSave(edge.id);
                                                }
                                            } }
                                        />
                                        <Button
                                            variant="main"
                                            square
                                            onClick={ () => handleSave(edge.id) }
                                            disabled={
                                                (editingValues[edge.id] ?? edge.state?.join(", ")) === edge.state?.join(", ")
                                            }
                                        >
                                            <Save />
                                        </Button>
                                    </div>
                                </div>
                            )}
                        </div>
                    ),
                )
            )}
            <Button variant="main">
                <Plus />
            </Button>
        </>
    );
}