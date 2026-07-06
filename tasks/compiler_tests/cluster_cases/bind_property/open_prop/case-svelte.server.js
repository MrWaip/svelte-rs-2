import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let visible = $$props["visible"];
	$$renderer.push(`<details${$.attr("open", visible, true)}><summary>x</summary></details>`);
	$.bind_props($$props, { visible });
}
