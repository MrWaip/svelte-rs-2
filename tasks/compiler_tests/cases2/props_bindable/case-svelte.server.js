import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = void 0, count = 0 } = $$props;
		$$renderer.push(`<p>${$.escape(value)}</p> <p>${$.escape(count)}</p>`);
		$.bind_props($$props, {
			value,
			count
		});
	});
}
