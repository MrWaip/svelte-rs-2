import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		let items = $$props["items"];
		$$renderer.select({ value: foo }, ($$renderer) => {
			$$renderer.option({ value: items[0] }, ($$renderer) => {
				$$renderer.push(`a`);
			});
			$$renderer.option({ value: items[1] }, ($$renderer) => {
				$$renderer.push(`b`);
			});
		});
		$.bind_props($$props, {
			foo,
			items
		});
	});
}
