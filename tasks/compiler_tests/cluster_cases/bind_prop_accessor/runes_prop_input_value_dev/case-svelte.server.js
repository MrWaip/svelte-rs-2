import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value = "x" } = $$props;
	$$renderer.push(`<input${$.attr("value", value)}/>`);
}
