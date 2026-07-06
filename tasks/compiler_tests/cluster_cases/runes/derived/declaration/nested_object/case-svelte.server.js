import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = {
		p: { a: 1 },
		q: { b: 2 }
	};
	let a = $.derived(() => x.p.a), b = $.derived(() => x.q.b);
	$$renderer.push(`<button>${$.escape(a())}${$.escape(b())}</button>`);
}
