import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let cls = "a";
	let active = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class($.clsx(cls), "svelte-16bdf5m", { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
