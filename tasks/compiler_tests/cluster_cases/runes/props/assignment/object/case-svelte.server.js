import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 0, b = 0 } = $$props;
	function f() {
		({a, b} = {
			a: 1,
			b: 2
		});
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
