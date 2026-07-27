import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	function foo($$renderer) {
		$$renderer.push(`<b>hi</b>`);
	}
	$$renderer.push(`<div><span>${$.escape(x)}</span></div>`);
}
