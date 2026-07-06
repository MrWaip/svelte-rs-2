import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = {
		a: 1,
		b: 2
	};
	let a = $.derived(() => x.a), b = $.derived(() => x.b);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}</button>`);
}
