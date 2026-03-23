import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = 0;
	function getValues() {
		const doubled = $derived(x * 2);
		return {
			doubled,
			get live() {
				return doubled;
			}
		};
	}
	var $$exports = { getValues };
	return $.pop($$exports);
}
