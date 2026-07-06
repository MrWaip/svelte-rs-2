import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.push(`<div${$.attributes({ ...$$sanitized_props })}></div>`);
}
