import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function track(node) {
		return () => {};
	}
	$.attach($.document, () => track);
}
