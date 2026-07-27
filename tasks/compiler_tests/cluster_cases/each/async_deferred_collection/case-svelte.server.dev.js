import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function delay(value) {
			return Promise.resolve(value);
		}
		var loaded;
		var $$promises = $$renderer.run([async () => loaded = await delay([1, 2])]);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			const each_array = $.ensure_array_like(loaded);
			if (each_array.length !== 0) {
				$$renderer.push("<!--[-->");
				for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
					let item = each_array[$$index];
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 9, 1);
					$$renderer.push(`${$.escape(item)}</p>`);
					$.pop_element();
				}
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 11, 1);
				$$renderer.push(`empty</p>`);
				$.pop_element();
			}
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
