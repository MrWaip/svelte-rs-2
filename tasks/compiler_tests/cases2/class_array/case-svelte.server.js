import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = false;
	const base = "btn";
	$$renderer.push(`<div${$.attr_class($.clsx([
		base,
		active && "active",
		"extra"
	]))}>content</div>`);
}
