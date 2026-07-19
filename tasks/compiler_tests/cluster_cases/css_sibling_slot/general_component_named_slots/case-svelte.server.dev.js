App_1[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App_1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let App;
		$$renderer.push(`<div class="a svelte-1u1mcs6">`);
		$.push_element($$renderer, "div", 18, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` `);
		App($$renderer, { $$slots: {
			a: ($$renderer) => {
				$$renderer.push(`<div class="b svelte-1u1mcs6" slot="a">`);
				$.push_element($$renderer, "div", 20, 1);
				$$renderer.push(`</div>`);
				$.pop_element();
			},
			b: ($$renderer) => {
				$$renderer.push(`<div class="c" slot="b">`);
				$.push_element($$renderer, "div", 22, 1);
				$$renderer.push(`<div class="d svelte-1u1mcs6">`);
				$.push_element($$renderer, "div", 23, 2);
				$$renderer.push(`</div>`);
				$.pop_element();
				$$renderer.push(` <div class="e svelte-1u1mcs6">`);
				$.push_element($$renderer, "div", 24, 2);
				$$renderer.push(`</div>`);
				$.pop_element();
				$$renderer.push(`</div>`);
				$.pop_element();
			},
			c: ($$renderer) => {
				$$renderer.push(`<div class="f svelte-1u1mcs6" slot="c">`);
				$.push_element($$renderer, "div", 27, 1);
				$$renderer.push(`</div>`);
				$.pop_element();
			}
		} });
		$$renderer.push(`<!----> <div class="g svelte-1u1mcs6">`);
		$.push_element($$renderer, "div", 30, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App_1);
}
App_1.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App_1;
