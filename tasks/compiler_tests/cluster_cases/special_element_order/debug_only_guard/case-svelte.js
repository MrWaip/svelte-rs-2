import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = 0;
	$.template_effect(() => {
		console.log({ count: $.snapshot(count) });
		debugger;
	});
}
