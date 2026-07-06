import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	$$renderer.push(`<div>${$.escape(value)}</div>`);
	$.bind_props($$props, { value });
}
