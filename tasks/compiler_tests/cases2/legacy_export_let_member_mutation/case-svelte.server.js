import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $.fallback($$props["obj"], () => ({ a: 1 }), true);
		obj.a = 99;
		$$renderer.push(`<p>${$.escape(obj.a)}</p>`);
		$.bind_props($$props, { obj });
	});
}
