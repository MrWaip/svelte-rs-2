App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let first, second, total, b, d, e;
		let source = $$props["source"];
		$: ({users: [{name: first}, {name: second}], total} = source);
		$: ({a: [b, {c: [d, e]}]} = source);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.push(`${$.escape(first)}-${$.escape(second)}-${$.escape(total)}-${$.escape(b)}-${$.escape(d)}-${$.escape(e)}</p>`);
		$.pop_element();
		$.bind_props($$props, { source });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
