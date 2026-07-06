import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let phone;
		let challenge = $$props["challenge"];
		let error = $$props["error"];
		$: phone = $.store_get($$store_subs ??= {}, "$challenge", challenge)?.phone || "";
		$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$error", error))}/> <p>${$.escape(phone)}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			challenge,
			error
		});
	});
}
