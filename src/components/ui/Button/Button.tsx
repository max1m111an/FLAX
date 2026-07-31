import React from "react";
import clsx from "clsx";
import styles from "./Button.module.scss";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: "main" | "control";
    square?: boolean;
    fullWidth?: boolean;
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
    ({ className, variant = "main", square, fullWidth, children, ...props }, ref) => {
        return (
            <button
                ref={ ref }
                className={ clsx(
                    variant === "main" ? styles.mainBtn : styles.controlBtn,
                    square && styles.square,
                    fullWidth && styles.fullWidth,
                    className,
                ) }
                { ...props }
            >
                {children}
            </button>
        );
    },
);

Button.displayName = "Button";
