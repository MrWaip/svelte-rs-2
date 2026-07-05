import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
export default function App($$renderer) {
	let ref = void 0;
	let plainRef;
	Component($$renderer, {});
	$$renderer.push(`<!----> `);
	Component($$renderer, {});
	$$renderer.push(`<!----> `);
	Component($$renderer, {
		name: "test",
		children: ($$renderer) => {
			$$renderer.push(`<p>child content</p>`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!---->`);
}
