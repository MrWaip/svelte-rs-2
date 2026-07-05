import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b = 10, c = void 0, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<p>${$.escape(a)}</p> <p>${$.escape(b)}</p> <p>${$.escape(c)}</p>`);
		$.bind_props($$props, { c });
	});
}
