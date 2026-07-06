import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.select({ value: foo }, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
	$.bind_props($$props, { foo });
}
