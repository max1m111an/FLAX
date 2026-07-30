import { createNewNFA } from "@/api/nfaAPI.ts";
import styles from "@/scenes/MainScene.module.scss";

export const DebugScene = () => {
    const handleTest = async () => {
        try {
            const res = await createNewNFA("Test");
            alert(`Успех:\n${JSON.stringify(res, null, 2)}`);
        } catch (e) {
            alert(`Ошибка:\n${JSON.stringify(e, null, 2)}`);
        }
    };

    return (
        <div>
            <button className={ styles.controlButton } onClick={ handleTest }>create_new_nfa</button>
        </div>
    );
};