import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x: y = 0 } = $$props;
		$$renderer.push(`<button>${$.escape(y)}</button>`);
		$.bind_props($$props, { x: y });
	});
}
