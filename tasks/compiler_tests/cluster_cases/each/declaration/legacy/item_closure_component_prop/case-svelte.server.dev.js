App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let clicked = $$props["clicked"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(["x"]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let letter = each_array[$$index];
			Widget($$renderer, { handleClick: () => clicked = letter });
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { clicked });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
