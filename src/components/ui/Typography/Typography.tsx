import React from "react";
import clsx from "clsx";
import styles from "./Typography.module.scss";

interface TypographyProps extends React.HTMLAttributes<HTMLParagraphElement> {
    variant?: "title" | "pretitle" | "label";
}

export const Typography: React.FC<TypographyProps> = ({
    variant = "label",
    className,
    children,
    ...props
}) => {
    return (
        <p
            className={ clsx(
                variant === "title" && styles.modelTitle,
                variant === "pretitle" && styles.modelPretitle,
                variant === "label" && styles.stateLabel,
                className,
            ) }
            { ...props }
        >
            {children}
        </p>
    );
};
