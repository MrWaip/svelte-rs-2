import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let name = $$props["name"];
	let typed = $.fallback($$props["typed"], null);
	$$renderer.push(`<p>${$.escape(name)}</p> <p>${$.escape(typed)}</p>`);
	$.bind_props($$props, {
		name,
		typed
	});
}
