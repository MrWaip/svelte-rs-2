import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	function getSnapshot() {
		// svelte-ignore state_snapshot_uncloneable
		return $.snapshot(items);
	}
}
