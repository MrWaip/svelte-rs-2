import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tag = $$props["tag"];
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`hello`);
	});
	$.bind_props($$props, { tag });
}
