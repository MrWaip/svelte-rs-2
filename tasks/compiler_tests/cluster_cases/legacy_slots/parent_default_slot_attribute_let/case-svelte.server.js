import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	A($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { foo }) => {
			$$renderer.push(`<span slot="default">${$.escape(foo)}</span>`);
		} }
	});
}
