import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b = 2, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(rest.c)}</button>`);
	});
}
