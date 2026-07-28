import React from "react";
import clsx from "clsx";
import styles from "./Textfield.module.scss";

interface TextfieldProps extends React.InputHTMLAttributes<HTMLInputElement> {
    onEdge?: boolean;
}

export const Textfield = React.forwardRef<HTMLInputElement, TextfieldProps>(
    ({ className, onEdge, ...props }, ref) => {
        return (
            <input
                ref={ ref }
                className={ clsx(
                    styles.input,
                    onEdge && styles.onEdge,
                    className,
                ) }
                { ...props }
            />
        );
    },
);

Textfield.displayName = "Textfield";

type TextAreaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement>;

export const TextArea = React.forwardRef<HTMLTextAreaElement, TextAreaProps>(
    ({ className, ...props }, ref) => {
        return (
            <textarea
                ref={ ref }
                className={ clsx(styles.multilineInput, className) }
                { ...props }
            />
        );
    },
);

TextArea.displayName = "TextArea";
