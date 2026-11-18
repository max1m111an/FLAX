export type ApiResponse<T = Record<string, never>> = {
    status: number;
    message: string;
} & T;