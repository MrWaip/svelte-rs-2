import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	Child($$renderer, { prop: Math.PI });
	$.bind_props($$props, { x });
}
