App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		{
			$.prevent_snippet_stringification(body);
			function body($$renderer) {
				$.validate_snippet_args($$renderer);
				if (data?.flag) {
					$$renderer.push("<!--[0-->");
					$.prevent_snippet_stringification(inner);
					function inner($$renderer) {
						$.validate_snippet_args($$renderer);
						$$renderer.push(`<!---->${$.escape(data?.flag?.text)}`);
					}
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 9, 3);
					$$renderer.push(`</div>`);
					$.pop_element();
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			}
			Outer($$renderer, {
				body,
				$$slots: { body: true }
			});
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
