import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["name"]);
	let name = $$props["name"];
	$$renderer.push(`<p>name: ${$.escape(name)}</p> <p>${$.escape(JSON.stringify($$restProps))}</p>`);
	$.bind_props($$props, { name });
}
