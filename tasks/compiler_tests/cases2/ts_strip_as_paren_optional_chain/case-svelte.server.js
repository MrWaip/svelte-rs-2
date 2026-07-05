import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		const z = (obj?.x)?.y;
		$$renderer.push(`<p>${$.escape(z)}</p>`);
		$.bind_props($$props, { obj });
	});
}
