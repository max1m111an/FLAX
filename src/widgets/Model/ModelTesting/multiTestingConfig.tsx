import React from "react";
import clsx from "clsx";
import styles from "./ModelTestWidget.module.scss";
import { LineTest, LineTestFA, LineTestMealy } from "@/types/Testing.ts";

const isFA = (trace: LineTest): trace is LineTestFA => "isFinal" in trace;
const isMealy = (trace: LineTest): trace is LineTestMealy => "output" in trace;

export interface TableConfig {
    header: string;
    getRowClass: (trace: LineTest) => string;
    renderInputCell: (trace: LineTest) => React.ReactNode;
    renderResultCell: (trace: LineTest) => React.ReactNode;
}

export const TABLE_CONFIGS: Record<string, TableConfig> = {
    FA: {
        header: "Статус",
        getRowClass: (trace) => {
            if (!isFA(trace)) return "";
            return trace.isFinal ? styles.rowAccepted : styles.rowRejected;
        },
        renderInputCell: (trace) => {
            if (!isFA(trace)) return null;
            return (
                <div className={ styles.charCell }>
                    {trace.line.split("").map((symbol, index) => (
                        <span key={ index }
                            className={ clsx(styles.resultChar, index < trace.correctSymbols ? styles.charSuccess : styles.charError) }>
                            {symbol}
                        </span>
                    ))}
                </div>
            );
        },
        renderResultCell: (trace) => {
            if (!isFA(trace)) return null;
            return (
                <td className={ trace.isFinal ? styles.statusAccepted : styles.statusRejected }>
                    {trace.isFinal ? "Принята" : "Отклонена"}
                </td>
            );
        },
    },
    MEALY: {
        header: "Вывод",
        getRowClass: (trace) => {
            if (!isMealy(trace)) return "";
            return trace.isComplete ? styles.rowAccepted : styles.rowRejected;
        },
        renderInputCell: (trace) => {
            if (!isMealy(trace)) return null;

            const textClass = trace.isComplete ? styles.charSuccess : styles.charError;

            return (
                <div className={ styles.charCell }>
                    {trace.line.split("").map((symbol, index) => (
                        <span key={ index } className={ clsx(styles.resultChar, textClass) }>
                            {symbol}
                        </span>
                    ))}
                </div>
            );
        },
        renderResultCell: (trace) => {
            if (!isMealy(trace)) return null;

            const statusClass = trace.isComplete ? styles.statusAccepted : styles.statusRejected;

            return (
                <td className={ clsx(styles.tableCell, statusClass) }>
                    {trace.output || "-"}
                </td>
            );
        },
    },
};