import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $.fallback($$props["obj"], () => ({ x: 0 }), true);
		$$renderer.push(`<!---->${$.escape(obj.x++)}`);
		$.bind_props($$props, { obj });
	});
}
