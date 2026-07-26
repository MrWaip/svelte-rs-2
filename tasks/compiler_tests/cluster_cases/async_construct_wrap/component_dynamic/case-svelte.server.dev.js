import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save(delay(x)))();
			if (Child) {
				$$renderer.push("<!--[-->");
				Child($$renderer, { value: $$0 });
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
