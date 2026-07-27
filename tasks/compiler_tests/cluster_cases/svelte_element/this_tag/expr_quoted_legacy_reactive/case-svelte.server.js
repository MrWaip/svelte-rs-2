import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tag;
	let n = $$props["n"];
	$: tag = "h" + n;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`hello`);
	});
	$.bind_props($$props, { n });
}
