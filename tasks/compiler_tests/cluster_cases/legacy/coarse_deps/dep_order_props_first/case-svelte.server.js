import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let id = $$props["id"];
		let callback = $$props["callback"];
		$: $$sanitized_props, callback(id);
		$.bind_props($$props, {
			id,
			callback
		});
	});
}
