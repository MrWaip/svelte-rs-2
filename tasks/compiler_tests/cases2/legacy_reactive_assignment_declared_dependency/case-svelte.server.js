import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let doubled, total;
	let count = 1;
	var step = 2;
	function bump() {
		count += 1;
		step += 1;
	}
	$: doubled = count * 2;
	$: total = doubled + step;
	$$renderer.push(`<button>${$.escape(doubled)}-${$.escape(total)}</button>`);
}
