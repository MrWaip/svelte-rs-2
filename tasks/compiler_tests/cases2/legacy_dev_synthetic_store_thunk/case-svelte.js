import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import { count } from "./stores";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get(count, "$count", $$stores);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = writable(0);
	$.init();
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, $count());
		$.set_text(text_1, $state());
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
