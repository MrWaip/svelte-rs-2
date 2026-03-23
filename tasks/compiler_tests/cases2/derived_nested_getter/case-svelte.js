import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = 0;
	function makeAccessor() {
		const computed = $.derived(() => value + 1);
		return { get computed() {
			return $.get(computed);
		} };
	}
	var $$exports = { makeAccessor };
	return $.pop($$exports);
}
