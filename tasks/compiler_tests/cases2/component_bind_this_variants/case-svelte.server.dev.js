App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [
			1,
			2,
			3
		];
		let refs = [];
		let obj = { ref: null };
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let item = each_array[i];
			Component($$renderer, {});
		}
		$$renderer.push(`<!--]--> `);
		Component($$renderer, {});
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
