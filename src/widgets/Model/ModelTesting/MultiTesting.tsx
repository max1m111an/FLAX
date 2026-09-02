import Play from "@/assets/svg/Play.svg?react";
import FileDown from "@/assets/svg/FileDown.svg?react";
import Uploud from "@/assets/svg/Uploud.svg?react";
import { Button } from "@/components/ui/Button/Button.tsx";
import { TextArea } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelTestWidget.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { lineTest, multiRunStrNFA, runStrNFA } from "@/api/nfaAPI.ts";
import Steps from "@/assets/svg/Steps.svg?react";
import clsx from "clsx";
import { useState } from "react";
import Reset from "@/assets/svg/Reset.svg?react";

export default function MultiTesting() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const [ isPlay, setIsPlay ] = useState(false);
    const [ traces, setTraces ] = useState<lineTest[]>([]);
    const [ isLoading, setIsLoading ] = useState<boolean>(false);
    const testLine = currentTab?.testInput ?? "";
    const setTestInput = (value: string) => {
        if (currentTab) {
            updateTab({ ...currentTab, testInput: value });
        }
    };

    if (!currentTab) return null;

    const handleOpenStep = async (line: string) => {
        const res = await runStrNFA({
            automatonId: currentTab!.id,
            input: line,
        });

        if ([ 200, 401, 402 ].includes(res.status)) {
            updateTab({
                ...currentTab!,
                testMode: "solo",
                pendingTestLine: line,
                pendingTraces: res.traces || [],
            });
        }
    };

    const handleMultiRun = async () => {
        setIsLoading(true);
        const request = {
            automatonId: currentTab?.id,
            inputs: testLine?.split(/\r?\n/) || [],
        };
        const response = await multiRunStrNFA(request);
        if (response.status == 200) {
            setTraces(response.traces);
            setIsPlay(true);
        }
        setIsLoading(false);
    };
    const handleReset = () => {
        setIsPlay(false);
    };
    return (
        <>
            <Typography variant="pretitle">Входные строки</Typography>
            <TextArea
                rows={ 5 }
                value={ testLine }
                disabled={ isPlay }
                onChange={ (e: React.ChangeEvent<HTMLTextAreaElement>) => {
                    setTestInput(e.target.value);
                } }
            />
            {
                !isPlay ? (
                    <>
                        <Button variant="main" onClick={ handleMultiRun }>
                            <Play />
                            Запустить все
                        </Button>
                        <div className={ styles.playExportWrapper }>
                            <Button variant="control" fullWidth>
                                <FileDown />
                                Экспорт
                            </Button>
                            <Button variant="control" fullWidth>
                                <Uploud />
                                Импорт
                            </Button>
                        </div>
                    </>
                ) : (
                    <Button variant="control" fullWidth onClick={ handleReset }>
                        <Reset />
                        Сброс
                    </Button>
                )
            }
            {
                isLoading && (
                    (<Typography variant="label">Прогоняем...</Typography>)
                )
            }
            {
                isPlay && (
                    <div>
                        <table className={ styles.multiTable }>
                            <thead className={ styles.tableHead }>
                                <tr>
                                    <th className={ styles.tableHeaderCell }>Строка</th>
                                    <th className={ styles.tableHeaderCell }>Статус</th>
                                    <th className={ clsx(styles.tableHeaderCell, styles.stepHeaderCell) }><Steps /></th>
                                </tr>
                            </thead>
                            <tbody className={ styles.tableBody }>
                                {traces.map((trace) => (
                                    <tr className={ clsx(styles.tableRow, trace.isFinal ? styles.rowAccepted : styles.rowRejected) }>
                                        <td className={ styles.tableCell }>
                                            <div className={ styles.charCell }>
                                                {trace.line.split("").map((symbol, index) => (
                                                    <span key={ index }
                                                        className={ clsx(styles.resultChar, index < trace.correctSymbols ? styles.charSuccess : styles.charError) }>{symbol}</span>
                                                ))}
                                            </div>
                                        </td>
                                        <td className={ trace.isFinal ? styles.statusAccepted : styles.statusRejected }>{trace.isFinal ? "Принята" : "Отклонена"}</td>
                                        <td className={ styles.stepCell }>
                                            <button className={ styles.openSoloBtn } onClick={ () => handleOpenStep(trace.line) }><Steps /></button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )
            }
        </>
    );
}
