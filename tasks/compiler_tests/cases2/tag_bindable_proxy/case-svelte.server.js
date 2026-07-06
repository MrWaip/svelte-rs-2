import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items = [
			1,
			2,
			3
		] } = $$props;
		$$renderer.push(`<p>${$.escape(items)}</p>`);
		$.bind_props($$props, { items });
	});
}
