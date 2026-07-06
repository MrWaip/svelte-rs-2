App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div class="svelte-19xqvng">`);
		$.push_element($$renderer, "div", 7, 0);
		$$renderer.push(`<style>
    .nested {
      color: red;
    }
  </style>`);
		$$renderer.push(` <p class="nested">`);
		$.push_element($$renderer, "p", 14, 2);
		$$renderer.push(`inside div</p>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` `);
		if (true) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<style>
    span {
      color: green;
    }
  </style>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
