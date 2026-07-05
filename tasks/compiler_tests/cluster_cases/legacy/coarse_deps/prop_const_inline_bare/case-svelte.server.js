import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	const k = 1;
	Child($$renderer, { prop: k });
	$.bind_props($$props, { x });
}
