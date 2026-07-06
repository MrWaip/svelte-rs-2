import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let indeterminate = false;
	let open = true;
	$$renderer.push(`<input type="checkbox"/> <details${$.attr("open", open, true)}></details>`);
}
