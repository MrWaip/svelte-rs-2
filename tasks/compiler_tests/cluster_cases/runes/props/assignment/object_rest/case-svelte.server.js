import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a = 0 } = $$props;
	function f() {
		({a, ...rest} = {
			a: 1,
			b: 2
		});
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
}
