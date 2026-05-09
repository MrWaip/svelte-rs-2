import type Default from 'mod1';
import type { TypeOnly } from 'mod2';
import { type Skipped, real } from 'mod3';
export type ExportedAlias = number;
export { type SkippedExp } from 'mod3';

export function f1(x?: string): void {
	console.log(x, real);
}

export function f2(...xs: number[]): number {
	return xs.length;
}

declare const obj: { a?: { b?: { c?: number } } };
export const v1 = obj.a?.b!.c;

declare const counter: { n: number | null };
export function f3(): void {
	counter.n!++;
	counter.n! = 5;
}

declare global {
	interface Window { foo: number }
}

export class A<T> implements Disposable {
	[Symbol.dispose](): void {}
	concrete: T;
	constructor(v: T) { this.concrete = v; }
}
