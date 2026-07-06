import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = [
		1,
		2,
		3
	];
	function getTotal() {
		const total = $.derived(() => {
			let sum = 0;
			for (const item of items) {
				sum += item;
			}
			return sum;
		});
		return total();
	}
	$.bind_props($$props, { getTotal });
}
