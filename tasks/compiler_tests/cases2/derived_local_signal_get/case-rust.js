import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = 0;
	function getValues() {
		const doubled = $.derived(() => x * 2);
		return {
			doubled: $.get(doubled),
			get live() {
				return $.get(doubled);
			}
		};
	}
	var $$exports = { getValues };
	return $.pop($$exports);
}
