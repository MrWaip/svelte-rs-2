import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let config = $$props["config"];
		$$renderer.push(`<div${$.attr_class($.clsx(config.cls))}></div>`);
		$.bind_props($$props, { config });
	});
}
