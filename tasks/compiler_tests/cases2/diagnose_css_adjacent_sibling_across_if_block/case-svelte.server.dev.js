App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x, y } = $$props;
		$$renderer.push(`<div class="a svelte-13830z5">`);
		$.push_element($$renderer, "div", 5, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` `);
		if (x) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="b svelte-13830z5">`);
			$.push_element($$renderer, "div", 7, 2);
			$$renderer.push(`</div>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> `);
		if (y) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="c svelte-13830z5">`);
			$.push_element($$renderer, "div", 10, 2);
			$$renderer.push(`</div>`);
			$.pop_element();
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
