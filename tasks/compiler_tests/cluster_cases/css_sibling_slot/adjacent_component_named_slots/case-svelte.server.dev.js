App_1[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App_1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let App;
		$$renderer.push(`<div class="a svelte-v8ftti">`);
		$.push_element($$renderer, "div", 14, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` `);
		App($$renderer, { $$slots: {
			a: ($$renderer) => {
				$$renderer.push(`<div class="b svelte-v8ftti" slot="a">`);
				$.push_element($$renderer, "div", 16, 1);
				$$renderer.push(`</div>`);
				$.pop_element();
			},
			b: ($$renderer) => {
				$$renderer.push(`<div class="c svelte-v8ftti" slot="b">`);
				$.push_element($$renderer, "div", 18, 1);
				$$renderer.push(`<div class="d svelte-v8ftti">`);
				$.push_element($$renderer, "div", 19, 2);
				$$renderer.push(`</div>`);
				$.pop_element();
				$$renderer.push(` <div class="e svelte-v8ftti">`);
				$.push_element($$renderer, "div", 20, 2);
				$$renderer.push(`</div>`);
				$.pop_element();
				$$renderer.push(`</div>`);
				$.pop_element();
			}
		} });
		$$renderer.push(`<!----> <div class="f svelte-v8ftti">`);
		$.push_element($$renderer, "div", 24, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App_1);
}
App_1.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App_1;
