import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = false;
	let highlighted = true;
	const base = "btn";
	$$renderer.push(`<div${$.attr_class($.clsx([base, active && "active"]), void 0, { "highlighted": highlighted })}>content</div>`);
}
