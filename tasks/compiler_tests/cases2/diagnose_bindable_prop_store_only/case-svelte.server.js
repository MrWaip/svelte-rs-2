import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { inputValue = void 0 } = $$props;
		function set5() {
			inputValue.set(5);
		}
		$$renderer.push(`<button>read: ${$.escape($.store_get($$store_subs ??= {}, "$inputValue", inputValue))}</button> <input${$.attr("value", $.store_get($$store_subs ??= {}, "$inputValue", inputValue))}/>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { inputValue });
	});
}
