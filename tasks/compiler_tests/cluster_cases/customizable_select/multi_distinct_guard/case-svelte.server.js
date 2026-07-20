import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = "hi";
	$$renderer.push(`<select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<b>${$.escape(x)}</b>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select> <select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<i>${$.escape(x)}</i>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select> <button>x</button>`);
}
