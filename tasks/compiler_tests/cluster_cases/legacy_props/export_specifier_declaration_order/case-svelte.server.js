import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let n1 = $$props["n1"];
	let n2 = $$props["n2"];
	let s3 = $$props["a6"];
	$$renderer.push(`<!---->$s3=${$.escape($.store_get($$store_subs ??= {}, "$s3", s3))}
${$.escape(n1)}${$.escape(n2)}`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, {
		n1,
		n2,
		a6: s3
	});
}
