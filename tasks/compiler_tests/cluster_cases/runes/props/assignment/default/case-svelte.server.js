import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 0, b = 0 } = $$props;
	function f() {
		[a = 9, b = 9] = [1];
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
