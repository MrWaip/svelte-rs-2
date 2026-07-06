import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onCb = $.fallback($$props["onCb"], () => {});
	$$renderer.push(`<!---->${$.escape(onCb)}`);
	$.bind_props($$props, { onCb });
}
