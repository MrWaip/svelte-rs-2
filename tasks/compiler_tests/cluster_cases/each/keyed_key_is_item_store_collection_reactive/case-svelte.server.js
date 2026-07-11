import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { store, columns } = $$props;
		const s = $.derived(() => store);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$s", s()).keys);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let key = each_array[$$index];
			const column = columns[key];
			$$renderer.push(`<div>${$.escape(column)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
