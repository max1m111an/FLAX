import React from "react";
import clsx from "clsx";
import styles from "./Switch.module.scss";

interface SwitchProps extends React.InputHTMLAttributes<HTMLInputElement> {
    round?: boolean;
}

export const Switch = React.forwardRef<HTMLInputElement, SwitchProps>(
    ({ className, round = true, ...props }, ref) => {
        return (
            <label className={ clsx(styles.switch, className) }>
                <input type="checkbox" ref={ ref } { ...props } />
                <span className={ clsx(styles.slider, round && styles.round) }></span>
            </label>
        );
    },
);

Switch.displayName = "Switch";
