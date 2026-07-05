import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let kind = $.fallback($$props["kind"], "a");
	let label = $.fallback($$props["label"], kind === "a" ? "first" : "second");
	$$renderer.push(`<div>${$.escape(label)}</div>`);
	$.bind_props($$props, {
		kind,
		label
	});
}
