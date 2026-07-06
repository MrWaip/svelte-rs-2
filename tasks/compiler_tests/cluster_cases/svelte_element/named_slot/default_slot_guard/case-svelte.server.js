import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer, $$props) {
	let tag = $.fallback($$props["tag"], "h1");
	Foo($$renderer, {
		children: ($$renderer) => {
			$.element($$renderer, tag, void 0, () => {
				$$renderer.push(`This is default slot`);
			});
		},
		$$slots: { default: true }
	});
	$.bind_props($$props, { tag });
}
