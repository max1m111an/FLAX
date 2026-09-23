import React from "react";
import styles from "./ConfirmCloseModal.module.scss";

interface ConfirmCloseModalProps {
    title: string;
    message: string;
    onSave: () => void;
    onDiscard: () => void;
    onCancel: () => void;
}

export const ConfirmCloseModal: React.FC<ConfirmCloseModalProps> = ({
    title,
    message,
    onSave,
    onDiscard,
    onCancel,
}) => {
    return (
        <div className={ styles.overlay } onClick={ onCancel }>
            <div
                className={ styles.modal }
                role="alertdialog"
                aria-modal="true"
                aria-labelledby="confirm-close-title"
                onClick={ (e) => e.stopPropagation() }
            >
                <h2 id="confirm-close-title" className={ styles.title }>
                    {title}
                </h2>
                <p className={ styles.message }>{message}</p>
                <div className={ styles.actions }>
                    <button className={ styles.saveBtn } onClick={ onSave }>
                        Сохранить
                    </button>
                    <button className={ styles.discardBtn } onClick={ onDiscard }>
                        Не сохранять
                    </button>
                    <button className={ styles.cancelBtn } onClick={ onCancel } autoFocus>
                        Отмена
                    </button>
                </div>
            </div>
        </div>
    );
};