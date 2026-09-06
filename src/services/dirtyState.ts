let dirtySet = new Set<number>();
const listeners = new Set<() => void>();

const emit = () => {
    listeners.forEach((listener) => listener());
};

export const markDirty = (automatonId: number, modified: boolean): void => {
    const next = new Set(dirtySet);
    if (modified) {
        next.add(automatonId);
    } else {
        next.delete(automatonId);
    }

    if (next.size === dirtySet.size && [ ...next ].every((id) => dirtySet.has(id))) {
        return;
    }

    dirtySet = next;
    emit();
};

export const subscribeDirty = (listener: () => void): (() => void) => {
    listeners.add(listener);
    return () => {
        listeners.delete(listener);
    };
};

export const getDirtySnapshot = (): Set<number> => dirtySet;

export const isDirty = (automatonId: number): boolean => dirtySet.has(automatonId);

export const stripDirtyStar = (title: string): string =>
    title.endsWith("*") ? title.slice(0, -1) : title;
