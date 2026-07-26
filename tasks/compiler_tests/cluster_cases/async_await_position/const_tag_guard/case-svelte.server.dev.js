import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve({ value });
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like([1]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			let current;
			var promises = $$renderer.run([async () => current = (await $.save(delay(x)))().value]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 13, 1);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(current)));
			$$renderer.push(`${$.escape(item)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
