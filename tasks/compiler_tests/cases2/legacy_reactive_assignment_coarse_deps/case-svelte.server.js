import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["items", "extra"]);
	$$renderer.component(($$renderer) => {
		let prop_total, props_items, rest_class;
		let items = $.fallback($$props["items"], () => [{ value: 1 }], true);
		let extra = $.fallback($$props["extra"], 2);
		$: prop_total = items[0].value + extra;
		$: props_items = $$sanitized_props.items[0].value;
		$: rest_class = $$restProps.class ?? "none";
		$$renderer.push(`<p class="x">${$.escape(prop_total)}-${$.escape(props_items)}-${$.escape(rest_class)}</p>`);
		$.bind_props($$props, {
			items,
			extra
		});
	});
}
