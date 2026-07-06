import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let data = {
		a: 1,
		b: 2
	};
	let $$d = $.derived(() => data), a = $.derived(() => $$d().a), b = $.derived(() => $$d().b);
	$$renderer.push(`<p>${$.escape(a())},${$.escape(b())}</p>`);
}
