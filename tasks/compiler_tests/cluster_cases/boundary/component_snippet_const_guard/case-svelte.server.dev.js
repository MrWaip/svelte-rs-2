App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { n } = $$props;
		function compute() {
			return n + 1;
		}
		{
			$.prevent_snippet_stringification(cell);
			function cell($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 14, 2);
				$$renderer.push(`cell</p>`);
				$.pop_element();
			}
			Row($$renderer, {
				cell,
				children: $.prevent_snippet_stringification(($$renderer) => {
					const value = compute();
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 11, 1);
					$$renderer.push(`${$.escape(value)}</div>`);
					$.pop_element();
				}),
				$$slots: {
					cell: true,
					default: true
				}
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
