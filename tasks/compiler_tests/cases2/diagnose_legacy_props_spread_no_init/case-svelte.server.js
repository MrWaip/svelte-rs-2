import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	let props;
	$: props = $$sanitized_props;
	$$renderer.push(`<div${$.attributes({ ...props })}></div>`);
}
