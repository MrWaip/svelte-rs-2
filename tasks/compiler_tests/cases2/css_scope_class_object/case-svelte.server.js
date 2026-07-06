import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = false;
	let big = false;
	$$renderer.push(`<div${$.attr_class($.clsx({
		active,
		big
	}), "svelte-az1y0o", { "extra": active })}>content</div>`);
}
