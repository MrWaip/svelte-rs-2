import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let obj = $.proxy({
		items: null,
		data: null,
		list: null,
		map: null
	});
	// Non-statement assignment — should use $.assign_nullish in dev
	(obj.items ??= []).push(1);
	// Non-statement assignment — should use $.assign in dev
	(obj.data = []).push(2);
	// Non-statement — $.assign_and
	(obj.list &&= []).length;
	// Non-statement — $.assign_or
	(obj.map ||= []).length;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, obj.items));
	$.append($$anchor, p);
	$.pop();
}
