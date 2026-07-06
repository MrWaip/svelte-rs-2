import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let selected = $$props["selected"];
	$$renderer.select({ value: selected }, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
	$.bind_props($$props, { selected });
}
