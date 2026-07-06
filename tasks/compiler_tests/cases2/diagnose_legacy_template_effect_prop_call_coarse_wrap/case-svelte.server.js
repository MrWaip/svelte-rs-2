import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let placeholder = $$props["placeholder"];
		let fieldData = $$props["fieldData"];
		$$renderer.push(`<input${$.attr("value", placeholder || fieldData.label)}/>`);
		$.bind_props($$props, {
			placeholder,
			fieldData
		});
	});
}
