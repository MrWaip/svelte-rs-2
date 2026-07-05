import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $.fallback($$props["foo"], () => [], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(Array(3));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			Foo($$renderer, {});
			$$renderer.push(`<!----> <div>${$.escape(i)} has foo: ${$.escape(!!foo[i])}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { foo });
	});
}
