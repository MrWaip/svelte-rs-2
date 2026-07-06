import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	// svelte-ignore state_snapshot_uncloneable
	let snap = items;
}
