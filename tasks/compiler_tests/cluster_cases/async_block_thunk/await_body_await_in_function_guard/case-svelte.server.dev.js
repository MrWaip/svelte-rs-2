import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		function call(callback) {
			return callback();
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 13, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			if ((await $.save(call(async () => await delay(x))))()) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 15, 44);
				$$renderer.push(`truthy</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <!--[--><!---->`);
		{
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 16, 45);
			$$renderer.push(`keyed</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!----><!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
