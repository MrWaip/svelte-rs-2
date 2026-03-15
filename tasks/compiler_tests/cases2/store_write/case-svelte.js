import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	$.store_set(count, 5);
	$.update_store(count, $count());
	$.update_pre_store(count, $count());
	$.update_store(count, $count(), -1);
	$.store_set(count, $count() + 10);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, p);
	$$cleanup();
}
