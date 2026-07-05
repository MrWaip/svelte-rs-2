import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	Foo($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { onClick }) => {
			$$renderer.push(`<p>${$.escape(onClick)}</p>`);
		} }
	});
}
