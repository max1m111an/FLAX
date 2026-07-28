import React from "react";
import clsx from "clsx";
import styles from "./IconButton.module.scss";

interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: "default" | "cancel";
}

export const IconButton = React.forwardRef<HTMLButtonElement, IconButtonProps>(
    ({ className, variant = "default", children, ...props }, ref) => {
        return (
            <button
                ref={ ref }
                className={ clsx(
                    styles.iconButton,
                    variant === "cancel" && styles.cancel,
                    className,
                ) }
                { ...props }
            >
                {children}
            </button>
        );
    },
);

IconButton.displayName = "IconButton";
