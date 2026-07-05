import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.select({ value: foo }, ($$renderer) => {
		$$renderer.option({
			value: null,
			disabled: true
		}, ($$renderer) => {
			$$renderer.push(`Select an option`);
		});
	});
	$.bind_props($$props, { foo });
}
