import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = "hi";
	$$renderer.push(`<select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<span>static</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select> <select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<span>${$.escape(x)}</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select> <button>x</button>`);
}
