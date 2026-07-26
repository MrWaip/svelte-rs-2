import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		function delay_list(value) {
			return Promise.resolve([value]);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 13, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			if ((await $.save(delay(x)))()) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 15, 20);
				$$renderer.push(`truthy</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <!--[-->`);
		$$renderer.child_block(async ($$renderer) => {
			const each_array = $.ensure_array_like((await $.save(delay_list(x)))());
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 16, 35);
				$$renderer.push(`${$.escape(item)}</p>`);
				$.pop_element();
			}
		});
		$$renderer.push(`<!--]--> <!--[--><!---->`);
		{
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 17, 21);
			$$renderer.push(`keyed</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!----><!--]--> `);
		$$renderer.child_block(async ($$renderer) => {
			$.await($$renderer, (async () => (await $.save(delay(x)))())(), () => {}, (value) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 18, 34);
				$$renderer.push(`${$.escape(value)}</p>`);
				$.pop_element();
			});
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
