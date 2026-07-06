import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {}, a = $.fallback(tmp.p, () => ({}), true).a;
	function bump() {
		a = a;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
