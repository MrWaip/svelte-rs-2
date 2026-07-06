import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 0, b = 0, c = 0, d = 0 } = $$props;
	function f() {
		[[a, b], [c, d]] = [[1, 2], [3, 4]];
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
}
