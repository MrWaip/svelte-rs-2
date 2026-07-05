import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = "a";
	$$renderer.push(`<select>`);
	$$renderer.option({ value: v }, ($$renderer) => {
		$$renderer.push(`<span>${$.escape(v)}</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select> <button>x</button>`);
}
