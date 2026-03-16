import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let obj = $.proxy({
		a: 1,
		b: 2
	});
	let snap = $.snapshot(obj);
}
