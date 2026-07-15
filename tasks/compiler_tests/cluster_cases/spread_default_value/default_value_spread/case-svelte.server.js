import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let spread = {};
	let v = void 0;
	$$renderer.push(`<input${$.attributes({
		value: v,
		...spread
	}, void 0, void 0, void 0, 4)}/>`);
}
