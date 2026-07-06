App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { foo } from "./utils";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(foo.bar);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let bar = each_array[$$index];
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 6, 1);
			$$renderer.push(`${$.escape(bar)}</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
