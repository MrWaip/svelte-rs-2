import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let y = $$props["y"];
	$$renderer.push(`<pre>${$.escape((1, ""))}</pre>`);
	$.bind_props($$props, { y });
}
