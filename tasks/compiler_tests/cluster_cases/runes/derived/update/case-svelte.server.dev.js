App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = $.derived(() => 0);
		let postfix = $.update_derived(count);
		let postfix_minus = $.update_derived(count, -1);
		let prefix = $.update_derived_pre(count);
		let prefix_minus = $.update_derived_pre(count, -1);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`${$.escape(postfix)}, ${$.escape(postfix_minus)}, ${$.escape(prefix)}, ${$.escape(prefix_minus)}, ${$.escape(count())}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
