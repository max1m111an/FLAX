import { addStateNFA, createNewNFA } from "@/api/nfaAPI.ts";
import styles from "@/scenes/MainScene.module.scss";
import { useState } from "react";

export const DebugScene = () => {
    const [ response, setResponse ] = useState("");
    return (
        <div>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await createNewNFA("Test");
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>create_new_nfa</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await addStateNFA({
                        automatonId: 1,
                        label: "q0",
                        x: 100,
                        y: 100,
                        isInitial: false,
                        isFinal: false,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_add_state</button>
            <pre
                className={ styles.cardDescriptionType }
                style={ {
                    backgroundColor: "#1e1e1e",
                    color: "#d4d4d4",
                    padding: "15px",
                    borderRadius: "8px",
                    overflowX: "auto",
                    fontFamily: "Consolas, monospace",
                    whiteSpace: "pre-wrap",
                    border: "1px solid #333",
                    userSelect: "text",
                } }
            >
                {response || "Ожидание ответа от бэкенда..."}
            </pre>
        </div>
    );
};