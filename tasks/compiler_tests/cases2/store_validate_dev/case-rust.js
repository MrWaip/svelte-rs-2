App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $count()));
	$.append($$anchor, p);
	return $.pop($$exports);
	$$cleanup();
}
