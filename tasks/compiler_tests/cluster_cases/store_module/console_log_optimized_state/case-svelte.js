import * as $ from "svelte/internal/client";
const value = 0;
$.effect_root(() => {
	$.user_effect(() => {
		console.log(value);
	});
});
