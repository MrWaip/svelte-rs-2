import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { h = 0 } = $$props;
		$$renderer.push(`<div></div>`);
		$.bind_props($$props, { h });
	});
}
