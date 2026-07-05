import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { selected = void 0 } = $$props;
		$$renderer.select({ value: selected }, ($$renderer) => {
			$$renderer.option({}, ($$renderer) => {
				$$renderer.push(`a`);
			});
			$$renderer.option({}, ($$renderer) => {
				$$renderer.push(`b`);
			});
		});
		$.bind_props($$props, { selected });
	});
}
