import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $$props["x"];
	Child($$renderer, { prop: $$slots.default });
	$.bind_props($$props, { x });
}
