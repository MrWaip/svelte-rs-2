import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let items = $.proxy([
		1,
		2,
		3
	]);
	function getTotal() {
		const total = $.derived(() => {
			let sum = 0;
			for (const item of items) {
				sum += item;
			}
			return sum;
		});
		return $.get(total);
	}
	var $$exports = { getTotal };
	return $.pop($$exports);
}
