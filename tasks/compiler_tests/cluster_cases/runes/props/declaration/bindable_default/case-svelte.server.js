import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x = 5 } = $$props;
		$$renderer.push(`<button>${$.escape(x)}</button>`);
		$.bind_props($$props, { x });
	});
}
