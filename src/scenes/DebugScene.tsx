import { addStateNFA, addTransitionNFA, createNewNFA, removeStateNFA, updateStateNFA } from "@/api/nfaAPI.ts";
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
                        automatonId: 2,
                        label: "q0",
                        x: 5455,
                        y: 555,
                        isInitial: false,
                        isFinal: false,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_add_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await removeStateNFA({
                        automatonId: 2,
                        stateId: 8335576,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_delete_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await updateStateNFA({
                        automatonId: 2,
                        stateId: 3159668,
                        label: "q4",
                        isFinal: true,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_update_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await addTransitionNFA({
                        automatonId: 2,
                        from: 3370849,
                        to: 5793196,
                        symbols: [ "d", "e", "f" ],
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_add_transition</button>

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