import * as $ from "svelte/internal/client";
export const make = () => {
	let x = 0;
	const s = $.derived(() => x + 1);
	return $.get(s);
};
