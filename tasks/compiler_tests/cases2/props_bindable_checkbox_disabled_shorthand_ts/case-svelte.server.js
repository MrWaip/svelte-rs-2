import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { checked = false, disabled = false } = $$props;
		$$renderer.push(`<input type="checkbox"${$.attr("checked", checked, true)}${$.attr("disabled", disabled, true)}/>`);
		$.bind_props($$props, { checked });
	});
}
