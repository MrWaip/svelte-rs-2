App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		{
			$.prevent_snippet_stringification(row);
			function row($$renderer, item) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 7, 2);
				$$renderer.push(`${$.escape(item)}</span>`);
				$.pop_element();
			}
			Table($$renderer, {
				items: data,
				row,
				$$slots: { row: true }
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
