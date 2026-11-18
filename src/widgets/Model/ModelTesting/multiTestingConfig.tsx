import React from "react";
import clsx from "clsx";
import styles from "./ModelTestWidget.module.scss";
import { lineTest, lineTestFA, lineTestMealy } from "@/types/Testing.ts";

const isFA = (trace: lineTest): trace is lineTestFA => "isFinal" in trace;
const isMealy = (trace: lineTest): trace is lineTestMealy => "output" in trace;

export interface TableConfig {
    header: string;
    getRowClass: (trace: lineTest) => string;
    renderInputCell: (trace: lineTest) => React.ReactNode;
    renderResultCell: (trace: lineTest) => React.ReactNode;
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