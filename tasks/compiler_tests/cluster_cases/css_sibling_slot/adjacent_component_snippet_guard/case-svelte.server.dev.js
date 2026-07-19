App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`<x class="svelte-zsofz2">`);
		$.push_element($$renderer, "x", 6, 1);
		$$renderer.push(`</x>`);
		$.pop_element();
		$$renderer.push(` `);
		{
			$.prevent_snippet_stringification(foo);
			function foo($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<v class="svelte-zsofz2">`);
				$.push_element($$renderer, "v", 10, 3);
				$$renderer.push(`</v>`);
				$.pop_element();
			}
			Child($$renderer, {
				foo,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<y class="svelte-zsofz2">`);
					$.push_element($$renderer, "y", 8, 2);
					$$renderer.push(`</y>`);
					$.pop_element();
				}),
				$$slots: {
					foo: true,
					default: true
				}
			});
		}
		$$renderer.push(`<!----> <z class="svelte-zsofz2">`);
		$.push_element($$renderer, "z", 13, 1);
		$$renderer.push(`</z>`);
		$.pop_element();
		$$renderer.push(` `);
		{
			$.prevent_snippet_stringification(foo);
			function foo($$renderer) {
				$.validate_snippet_args($$renderer);
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 20, 3);
				$$renderer.push(`<n>`);
				$.push_element($$renderer, "n", 21, 4);
				$$renderer.push(`</n>`);
				$.pop_element();
				$$renderer.push(`</span>`);
				$.pop_element();
			}
			Child($$renderer, {
				foo,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<span>`);
					$.push_element($$renderer, "span", 16, 2);
					$$renderer.push(`<n>`);
					$.push_element($$renderer, "n", 17, 3);
					$$renderer.push(`</n>`);
					$.pop_element();
					$$renderer.push(`</span>`);
					$.pop_element();
				}),
				$$slots: {
					foo: true,
					default: true
				}
			});
		}
		$$renderer.push(`<!----> <m>`);
		$.push_element($$renderer, "m", 25, 1);
		$$renderer.push(`</m>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
