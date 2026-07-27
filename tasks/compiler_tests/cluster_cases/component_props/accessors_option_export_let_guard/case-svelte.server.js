import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let stuff = $$props["stuff"];
	let count = $.fallback($$props["count"], 0);
	$.bind_props($$props, {
		stuff,
		count
	});
}
