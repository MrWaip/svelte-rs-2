import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			$.slot($$renderer, $$props, "default", { value: $$0 }, () => {
				$$renderer.push(`<b>`);
				$.push_element($$renderer, "b", 11, 29);
				$$renderer.push(`fallback</b>`);
				$.pop_element();
			});
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
