App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		const groups = $.derived(() => {
			const groups = new Map();
			for (const x of data.schema) groups.set(x, [{
				name: x,
				href: x
			}]);
			return groups;
		});
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(groups());
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let [group, links] = each_array[$$index_1];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 13, 4);
			$$renderer.push(`${$.escape(group)} <!--[-->`);
			const each_array_1 = $.ensure_array_like(links);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let { name, href } = each_array_1[$$index];
				$$renderer.push(`<a${$.attr("href", href)}>`);
				$.push_element($$renderer, "a", 16, 12);
				$$renderer.push(`${$.escape(name)}</a>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]--></div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
