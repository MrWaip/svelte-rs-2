App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = {
			label: "world",
			extra: "ok"
		};
		function key() {
			return "label";
		}
		$.prevent_snippet_stringification(view);
		function view($$renderer, { [key()]: value, ...rest }) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 13, 1);
			$$renderer.push(`${$.escape(value)} ${$.escape(rest.extra)}</p>`);
			$.pop_element();
		}
		view($$renderer, data);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
