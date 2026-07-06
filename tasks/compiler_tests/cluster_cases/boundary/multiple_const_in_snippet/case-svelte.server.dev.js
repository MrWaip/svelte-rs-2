App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { n } = $$props;
		function compute() {
			return n + 1;
		}
		$.prevent_snippet_stringification(failed);
		function failed($$renderer) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 14, 2);
			$$renderer.push(`failed</p>`);
			$.pop_element();
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				const a = compute();
				const b = compute();
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 11, 1);
				$$renderer.push(`${$.escape(a)}${$.escape(b)}</div>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
