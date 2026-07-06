import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const k = "z";
	let { a = 0 } = $$props;
	function f() {
		({[k]: a} = { z: 1 });
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
