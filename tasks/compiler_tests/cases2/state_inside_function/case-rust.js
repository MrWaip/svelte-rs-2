import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	function createCounter() {
		let count = $.state(0);
		return {
			get count() {
				return $.get(count);
			},
			increment() {
				$.update(count);
			}
		};
	}
	var $$exports = { createCounter };
	return $.pop($$exports);
}
