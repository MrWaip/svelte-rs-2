import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		$$renderer.push(`<details${$.attr("open", obj.flag, true)}><summary>x</summary></details>`);
		$.bind_props($$props, { obj });
	});
}
