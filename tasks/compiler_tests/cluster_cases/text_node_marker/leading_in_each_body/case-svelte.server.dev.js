App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let array = ["A"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(array);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let a = each_array[$$index];
			$$renderer.push(`<!---->${$.escape(a)}<br/>`);
			$.push_element($$renderer, "br", 5, 21);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`add</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
