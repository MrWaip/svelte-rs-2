App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(view);
function view($$renderer, { nested: { name = "fallback" }, list: [[first, ...rest]], ...tail }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 10, 1);
	$$renderer.push(`${$.escape(name)} ${$.escape(first)} ${$.escape(rest.length)} ${$.escape(tail.meta.note)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = {
			nested: { name: "world" },
			list: [[
				10,
				20,
				30
			]],
			meta: { note: "ok" }
		};
		view($$renderer, data);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
