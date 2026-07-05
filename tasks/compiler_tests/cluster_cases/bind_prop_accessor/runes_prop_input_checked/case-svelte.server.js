import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { checked = false } = $$props;
	$$renderer.push(`<input type="checkbox"${$.attr("checked", checked, true)}/>`);
}
