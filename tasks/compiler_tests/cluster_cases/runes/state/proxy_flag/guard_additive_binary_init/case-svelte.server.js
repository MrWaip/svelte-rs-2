import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	let sum = a + b;
	function inc() {
		sum = sum + 1;
	}
	$$renderer.push(`<button>${$.escape(sum)}</button>`);
}
