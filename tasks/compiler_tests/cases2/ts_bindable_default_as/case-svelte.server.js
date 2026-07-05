import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { val = 0 } = $$props;
		$$renderer.push(`<p>${$.escape(val)}</p>`);
		$.bind_props($$props, { val });
	});
}
