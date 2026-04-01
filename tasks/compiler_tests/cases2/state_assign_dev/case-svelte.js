App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({
		items: null,
		data: null,
		list: null,
		map: null
	}), "obj");
	// Non-statement assignment — should use $.assign_nullish in dev
	$.assign_nullish(obj, "items", [], "(unknown):5:2").push(1);
	// Non-statement assignment — should use $.assign in dev
	$.assign(obj, "data", [], "(unknown):8:2").push(2);
	// Non-statement — $.assign_and
	$.assign_and(obj, "list", [], "(unknown):11:2").length;
	// Non-statement — $.assign_or
	$.assign_or(obj, "map", [], "(unknown):14:2").length;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, obj.items));
	$.append($$anchor, p);
	return $.pop($$exports);
}
