import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	function getSnapshot() {
		// svelte-ignore state_snapshot_uncloneable
		return $.snapshot(items);
	}
}
