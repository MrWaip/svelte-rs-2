import * as $ from "svelte/internal/client";
const state = 0;
let inspect = {};
$.effect_root(() => {
	$.inspect(() => [state], (...$$args) => console.log(...$$args), true);
});
