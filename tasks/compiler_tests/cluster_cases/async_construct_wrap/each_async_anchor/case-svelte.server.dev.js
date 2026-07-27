import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let deferred = Promise.withResolvers();
		$$renderer.push(`<ul>`);
		$.push_element($$renderer, "ul", 5, 0);
		$$renderer.push(`<!--[-->`);
		$$renderer.child_block(async ($$renderer) => {
			const each_array = $.ensure_array_like((await $.save(deferred.promise))());
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				$$renderer.push(`<li>`);
				$.push_element($$renderer, "li", 7, 2);
				$$renderer.push(`${$.escape(item)}</li>`);
				$.pop_element();
			}
		});
		$$renderer.push(`<!--]--></ul>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
