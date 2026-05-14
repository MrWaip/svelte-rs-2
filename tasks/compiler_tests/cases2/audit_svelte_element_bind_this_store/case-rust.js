import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $store = () => $.store_get(store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let tag = "div";
	const store = writable();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.bind_this($$element, ($$value) => $.store_set(store, $$value), () => $store());
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
