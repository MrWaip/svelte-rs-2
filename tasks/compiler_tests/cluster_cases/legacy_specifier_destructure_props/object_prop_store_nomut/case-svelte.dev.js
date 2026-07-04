import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $c = () => ($.validate_store(c(), "c"), $.store_get(c(), "$c", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let tmp = {
		a: 1,
		c: writable(2)
	}, a = $.prop($$props, "a", 24, () => tmp.a), c = $.prop($$props, "c", 24, () => tmp.c);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$c() ?? ""}`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
