import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.b)}</button>`);
	});
}
