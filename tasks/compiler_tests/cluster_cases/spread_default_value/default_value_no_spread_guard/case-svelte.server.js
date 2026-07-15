import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = void 0;
	$$renderer.push(`<input${$.attr("value", v)}/>`);
}
