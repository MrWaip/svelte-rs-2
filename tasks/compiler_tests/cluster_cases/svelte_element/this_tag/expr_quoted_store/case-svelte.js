import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $tag = () => $.store_get(tag, "$tag", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const tag = writable("div");
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, $tag, false, ($$element, $$anchor) => {
		var text = $.text("hello");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
