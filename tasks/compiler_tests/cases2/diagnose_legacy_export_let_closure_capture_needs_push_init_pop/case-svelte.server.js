import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let duration = $$props["duration"];
		const opts = () => ({ duration });
		$$renderer.push(`<p>${$.escape(opts().duration)}</p>`);
		$.bind_props($$props, { duration });
	});
}
