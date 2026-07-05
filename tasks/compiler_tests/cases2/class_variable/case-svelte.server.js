import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cls = "primary";
	$$renderer.push(`<div${$.attr_class($.clsx(cls))}>content</div>`);
}
