import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = "x";
	$$renderer.push(`<select>`);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`<span>${$.escape(v)}</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.option({ value: "b" }, ($$renderer) => {
		$$renderer.push(`B`);
	});
	$$renderer.push(`</select> <button>x</button>`);
}
