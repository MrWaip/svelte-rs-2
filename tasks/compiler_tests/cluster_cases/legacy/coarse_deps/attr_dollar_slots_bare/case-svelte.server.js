import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $$props["x"];
	$$renderer.push(`<p${$.attr("hidden", $$slots)}>a</p>`);
	$.bind_props($$props, { x });
}
