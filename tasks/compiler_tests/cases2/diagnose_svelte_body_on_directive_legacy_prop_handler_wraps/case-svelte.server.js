import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onClick = $$props["onClick"];
	$.bind_props($$props, { onClick });
}
