declare const a: { b?: { c?: number } };

export const v = a.b?.c!;
