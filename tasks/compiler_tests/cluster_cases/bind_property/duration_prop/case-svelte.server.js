import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let d = $$props["d"];
	$$renderer.push(`<video></video>`);
	$.bind_props($$props, { d });
}
