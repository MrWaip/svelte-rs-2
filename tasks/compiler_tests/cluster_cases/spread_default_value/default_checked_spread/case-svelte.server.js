import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let spread = {};
	let c = void 0;
	$$renderer.push(`<input${$.attributes({
		type: "checkbox",
		checked: c,
		...spread
	}, void 0, void 0, void 0, 4)}/>`);
}
