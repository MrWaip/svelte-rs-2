import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function flush() {}
	$.event("obank-tab-stop", $.window, () => flush());
}
