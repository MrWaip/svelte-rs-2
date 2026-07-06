App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(foo.bar);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let bar = each_array[$$index];
			$$renderer.push(`<input${$.attr("value", bar)}/>`);
			$.push_element($$renderer, "input", 6, 1);
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
