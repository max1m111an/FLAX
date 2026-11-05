import React from "react";
import Play from "@/assets/svg/Play.svg?react";
import FastForward from "@/assets/svg/FastForward.svg?react";
import Back from "@/assets/svg/Back.svg?react";
import Forward from "@/assets/svg/Forward.svg?react";
import Reset from "@/assets/svg/Reset.svg?react";
import { Button } from "@/components/ui/Button/Button.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { TraceCard } from "@/components/TraceCard/TraceCard.tsx";
import { useTesting } from "@/hooks/useTesting.tsx";

export default function SoloTesting() {
    const {
        testLine,
        setTestLine,
        isPlay,
        currentIndex,
        symbols,
        traces,
        currentTab,
        handlePlay,
        handleStep,
        handleStepBack,
        handleFastForward,
        handleReset,
    } = useTesting();

    if (!currentTab || currentTab.type === "SETTINGS") return null;

    const displayTraces = traces.length > 0 ? traces : [ { steps: [], isFinal: false } ];

    return (
        <>
            <Typography variant="pretitle">Входная строка</Typography>

            <Textfield
                value={ testLine }
                disabled={ isPlay }
                onChange={ (e: React.ChangeEvent<HTMLInputElement>) => {
                    setTestLine(e.target.value);
                } }
            />

            {!isPlay && (
                <Button variant="main" onClick={ handlePlay }>
                    <Play />
                    Запустить
                </Button>
            )}

            {isPlay && (
                <>
                    <div style={ { display: "flex", flexDirection: "row", gap: "8px" } }>
                        <Button
                            variant="control"
                            onClick={ handleStep }
                            disabled={ currentIndex === symbols.length }
                        >
                            <Forward />
                            Шаг вперед
                        </Button>
                        <Button variant="control" onClick={ handleStepBack } disabled={ currentIndex === 0 }>
                            <Back />
                            Шаг назад
                        </Button>
                    </div>

                    <div style={ { display: "flex", flexDirection: "row", gap: "8px" } }>
                        <Button variant="control" fullWidth onClick={ handleFastForward } disabled={ currentIndex === symbols.length }>
                            <FastForward />
                            До конца
                        </Button>

                        <Button variant="control" fullWidth onClick={ handleReset }>
                            <Reset />
                            Сброс
                        </Button>
                    </div>

                    {displayTraces.map((traceObj, i) => {
                        return (
                            <TraceCard
                                key={ i }
                                traceSteps={ traceObj }
                                currentIndex={ currentIndex }
                                testLineLength={ testLine.length }
                                symbols={ symbols }
                                currentAutomaton={ currentTab.automaton }
                                type={ currentTab.type }
                            />
                        );
                    })}
                </>
            )}
        </>
    );
}