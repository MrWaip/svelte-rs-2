import * as $ from "svelte/internal/server";
function show($$renderer, a, b) {
	$$renderer.push(`<p>${$.escape(a)} ${$.escape(b)}</p>`);
}
export default function App($$renderer) {
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	show($$renderer, fn1(), fn2());
}
