import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let prop = $$props["prop"];
	let local = 0;
	let out = 0;
	$: if (true) {
		local = 1;
		out = (prop || 0) + local;
	}
	$$renderer.push(`<p>${$.escape(out)}</p>`);
	$.bind_props($$props, { prop });
}
