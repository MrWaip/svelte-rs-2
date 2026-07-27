import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [1, 2];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			let loaded;
			var promises = $$renderer.run([async () => loaded = (await $.save(Promise.resolve(item)))()]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 7, 1);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded)));
			$$renderer.push(`</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
