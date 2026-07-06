import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 0 } = $$props;
	function f() {
		[a, ...rest] = [
			1,
			2,
			3
		];
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
}
