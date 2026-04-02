import * as $ from "svelte/internal/client";
const view = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.().list, 1));
	var $$array_1 = $.derived(() => $.to_array($$array[0]));
	let name = $.derived_safe_equal(() => $.fallback($$arg0?.().nested.name, "fallback"));
	let first = () => $.get($$array_1)[0];
	let rest = () => $.get($$array_1).slice(1);
	let tail = () => $.exclude_from_object($$arg0?.(), ["nested", "list"]);
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} ${first() ?? ""} ${rest().length ?? ""} ${tail().meta.note ?? ""}`));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let data = $.proxy({
		nested: { name: "world" },
		list: [[
			10,
			20,
			30
		]],
		meta: { note: "ok" }
	});
	view($$anchor, () => data);
}
