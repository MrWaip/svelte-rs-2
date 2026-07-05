import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let cls = "a";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class($.clsx(cls))}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
