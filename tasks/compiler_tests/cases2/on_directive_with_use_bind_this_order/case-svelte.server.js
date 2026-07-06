import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let action = $$props["action"];
	let onClick = $$props["onClick"];
	let onKey = $$props["onKey"];
	let el;
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, {
		action,
		onClick,
		onKey
	});
}
