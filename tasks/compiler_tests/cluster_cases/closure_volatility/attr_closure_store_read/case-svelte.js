import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const count = writable(0);
	$.init();
	var div = root();
	$.set_attribute(div, "title", [() => $count()]);
	$.append($$anchor, div);
	$.pop();
	$$cleanup();
}
