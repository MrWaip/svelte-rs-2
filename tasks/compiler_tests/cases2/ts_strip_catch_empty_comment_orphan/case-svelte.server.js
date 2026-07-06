import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let n = $$props["n"];
	function run() {
		try {
			const a = 1;
			const b = 2;
			console.log(a, b);
		} catch {}
	}
	if (n) {
		console.log(n);
	}
	$$renderer.push(`<button>${$.escape(n)}</button>`);
	$.bind_props($$props, { n });
}
