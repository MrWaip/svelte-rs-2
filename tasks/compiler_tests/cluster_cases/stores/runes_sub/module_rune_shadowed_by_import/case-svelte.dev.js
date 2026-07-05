import * as $ from "svelte/internal/client";
import { derived } from "svelte/store";
export const make = () => {
	let x = 0;
	const s = $.tag($.derived(() => x + 1), "s");
	return $.get(s);
};
