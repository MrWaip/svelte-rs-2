import * as $ from "svelte/internal/client";
import { lazy } from "./lazy.svelte";
export const makeStore = () => {
	const data = lazy(null, async () => ({ flags: { a: true } }));
	const flagA = $.derived(() => Boolean(data?.value?.flags?.a));
	return { get flagA() {
		return $.get(flagA);
	} };
};
