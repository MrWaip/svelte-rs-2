import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { format = (x) => x, value = void 0, extra } = $$props;
		$$renderer.push(`<span>${$.escape(format(value))}${$.escape(extra)}</span>`);
		$.bind_props($$props, { value });
	});
}
