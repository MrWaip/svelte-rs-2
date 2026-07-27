import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve({
				flag: value,
				list: [value]
			});
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			if ((await $.save((await $.save(delay(x)))().flag))()) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 11, 33);
				$$renderer.push(`truthy</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <!--[-->`);
		$$renderer.child_block(async ($$renderer) => {
			const each_array = $.ensure_array_like((await $.save((await $.save(delay(x)))().list))());
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 12, 43);
				$$renderer.push(`${$.escape(item)}</p>`);
				$.pop_element();
			}
		});
		$$renderer.push(`<!--]--> <!--[--><!---->`);
		{
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 13, 34);
			$$renderer.push(`keyed</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!----><!--]--> `);
		$$renderer.child_block(async ($$renderer) => {
			$.await($$renderer, (async () => (await $.save((await $.save(delay(x)))().flag))())(), () => {}, (value) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 14, 47);
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
