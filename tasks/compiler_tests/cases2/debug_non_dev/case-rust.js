import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let x = 1;
	let y = 2;
	$.template_effect(() => {
		console.log({
			x: $.snapshot(x),
			y: $.snapshot(y)
		});
		debugger;
	});
}
