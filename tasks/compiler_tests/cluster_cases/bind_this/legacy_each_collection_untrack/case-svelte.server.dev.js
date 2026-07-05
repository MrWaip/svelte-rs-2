App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $.fallback($$props["foo"], () => [], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(Array(3));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			Foo($$renderer, {});
			$$renderer.push(`<!----> <div>`);
			$.push_element($$renderer, "div", 8, 1);
			$$renderer.push(`${$.escape(i)} has foo: ${$.escape(!!foo[i])}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
