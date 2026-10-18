import styles from "./Snackbar.module.scss";
import CircleAlert from "@/assets/svg/CircleAlert.svg?react";
import CircleMinus from "@/assets/svg/CircleMinus.svg?react";
import CircleCheck from "@/assets/svg/CircleCheck.svg?react";
import React, { useEffect, useState } from "react";
import clsx from "clsx";
import { createPortal } from "react-dom";

interface Snackbar {
    type?: "warning" | "success" | "error";
    message: string;
    onClose: () => void;
}
export const Snackbar: React.FC<Snackbar> = ({ type, message, onClose }: Snackbar) => {
    const [ isClosing, setIsClosing ] = useState(false);

    useEffect(() => {
        const timer = setTimeout(() => {
            setIsClosing(true);
        }, 3000);
        return () => clearTimeout(timer);
    }, []);

    return createPortal(
        <div
            className={ clsx(
                styles.modelWrapper,
                type && styles[type],
                isClosing && styles.closing) }
            onAnimationEnd={ () => {
                if (isClosing) {
                    onClose();
                }
            } }
        >
            {
                type === "success" && <CircleCheck className={ styles.icon } />
            }
            {
                type === "warning" && <CircleAlert className={ styles.icon } />
            }
            {
                type === "error" && <CircleMinus className={ styles.icon } />
            }

            <p >{message}</p>
        </div>,
        document.body,
    );
};