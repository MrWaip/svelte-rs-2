import * as $ from "svelte/internal/server";
export const make = () => {
	let x = 0;
	const s = $.derived(() => x + 1);
	return s();
};
