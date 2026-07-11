import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const DEFAULTS = { a: 1 };
		let { config = DEFAULTS } = $$props;
		$$renderer.push(`<button>${$.escape(config.a)}</button>`);
		$.bind_props($$props, { config });
	});
}
