import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let active = $$props["active"];
	Outer($$renderer, { $$slots: { activator: ($$renderer) => {
		$$renderer.push(`<div slot="activator"${$.attr_class("", void 0, { "active": active })}>hi</div>`);
	} } });
	$.bind_props($$props, { active });
}
