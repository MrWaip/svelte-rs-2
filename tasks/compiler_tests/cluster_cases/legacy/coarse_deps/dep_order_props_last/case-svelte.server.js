import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let id = $$props["id"];
		let callback = $$props["callback"];
		$: callback(id), $$sanitized_props;
		$.bind_props($$props, {
			id,
			callback
		});
	});
}
