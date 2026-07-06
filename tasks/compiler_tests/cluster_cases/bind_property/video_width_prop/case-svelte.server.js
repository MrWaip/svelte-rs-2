import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let w = $$props["w"];
	$$renderer.push(`<video></video>`);
	$.bind_props($$props, { w });
}
