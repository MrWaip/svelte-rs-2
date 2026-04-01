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
	(obj.items ??= []).push(1);
	// Non-statement assignment — should use $.assign in dev
	(obj.data = []).push(2);
	// Non-statement — $.assign_and
	(obj.list &&= []).length;
	// Non-statement — $.assign_or
	(obj.map ||= []).length;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, obj.items));
	$.append($$anchor, p);
	return $.pop($$exports);
}
