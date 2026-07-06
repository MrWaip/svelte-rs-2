import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		p: { a: 1 },
		q: { b: 2 }
	}, a = tmp.p.a, b = tmp.q.b;
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
