import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const total = 42;
	$$renderer.push(`<p>42</p>`);
	$.bind_props($$props, { total });
}
