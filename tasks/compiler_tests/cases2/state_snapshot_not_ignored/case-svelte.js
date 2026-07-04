import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	let snap = $.snapshot(items);
}
