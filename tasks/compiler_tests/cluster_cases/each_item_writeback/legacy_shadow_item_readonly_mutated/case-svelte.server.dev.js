App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = ["Hello"];
		function go() {
			a = [...a, "x"];
		}
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(a);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let a = each_array[$$index];
			$$renderer.push(`<!---->${$.escape(a)}`);
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
