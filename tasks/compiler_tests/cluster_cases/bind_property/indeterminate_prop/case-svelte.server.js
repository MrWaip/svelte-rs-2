import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let indeterminate = $$props["indeterminate"];
	$$renderer.push(`<input type="checkbox"/>`);
	$.bind_props($$props, { indeterminate });
}
