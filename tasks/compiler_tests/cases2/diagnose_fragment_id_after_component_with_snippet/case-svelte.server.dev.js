App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
import B from "./B.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = null;
		let x = null;
		{
			$.prevent_snippet_stringification(inner);
			function inner($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<!---->`);
			}
			A($$renderer, {
				inner,
				$$slots: { inner: true }
			});
		}
		$$renderer.push(`<!----> `);
		B($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				if (data) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<div>`);
					$.push_element($$renderer, "div", 16, 2);
					$$renderer.push(`c</div>`);
					$.pop_element();
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			}),
			$$slots: { default: true }
		});
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
