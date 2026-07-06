import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let files = $.fallback($$props["files"], undefined);
	$$renderer.push(`<input type="file" multiple=""/>`);
	$.bind_props($$props, { files });
}
