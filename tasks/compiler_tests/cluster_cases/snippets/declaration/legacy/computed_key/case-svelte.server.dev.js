App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const k = "z";
		let v = { z: 1 };
		$.prevent_snippet_stringification(s);
		function s($$renderer, { [k]: v }) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 7, 1);
			$$renderer.push(`${$.escape(v)}</button>`);
			$.pop_element();
		}
		s($$renderer, v);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
