import React, { createContext, useContext, useState } from "react";
import { Snackbar } from "@/components/Snackbar/Snackbar";
interface AlertContextType {
    showSnackbar: (type: "warning" | "success" | "error", message: string) => void;
}

const SnackbarContext = createContext<AlertContextType | null>(null);

export const SnackbarProvider = ({ children }: { children: React.ReactNode }) => {
    const [ alertData, setAlertData ] = useState<{
        id: number;
        type: "warning" | "success" | "error";
        message: string;
    } | null>(null);

    const showSnackbar = (type: "warning" | "success" | "error", message: string) => {
        setAlertData({ id: Date.now(), type, message });
    };

    return (
        <SnackbarContext.Provider value={ { showSnackbar } }>
            {children}
            {alertData && (
                <Snackbar
                    key={ alertData.id }
                    type={ alertData.type }
                    message={ alertData.message }
                    onClose={ () => setAlertData(null) }
                />
            )}
        </SnackbarContext.Provider>
    );
};

// eslint-disable-next-line react-refresh/only-export-components
export const useSnackbar = () => {
    const context = useContext(SnackbarContext);
    if (!context) throw new Error("useAlert must be used within AlertProvider");
    return context;
};