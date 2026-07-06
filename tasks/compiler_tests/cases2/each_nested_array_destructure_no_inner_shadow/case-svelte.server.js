import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
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
			$$renderer.push(`<div>${$.escape(group)} <!--[-->`);
			const each_array_1 = $.ensure_array_like(links);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let { name, href } = each_array_1[$$index];
				$$renderer.push(`<a${$.attr("href", href)}>${$.escape(name)}</a>`);
			}
			$$renderer.push(`<!--]--></div>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
