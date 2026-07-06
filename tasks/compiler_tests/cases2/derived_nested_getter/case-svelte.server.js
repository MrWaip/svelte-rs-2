import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = 0;
	function makeAccessor() {
		const computed = $.derived(() => value + 1);
		return { get computed() {
			return computed();
		} };
	}
	$.bind_props($$props, { makeAccessor });
}
