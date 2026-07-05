import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	$$renderer.push(`<input${$.attr("value", x)}/>`);
}
