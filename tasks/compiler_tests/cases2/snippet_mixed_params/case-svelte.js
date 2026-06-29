import * as $ from "svelte/internal/client";
const row = ($$anchor, label = $.noop, $$arg1, $$arg2) => {
	let id = () => $$arg1?.().id;
	var $$array = $.derived(() => $.to_array($$arg2?.(), 1));
	let value = () => $.get($$array)[0];
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${label() ?? ""}: ${id() ?? ""} = ${value() ?? ""}`));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([{ id: 1 }]);
	row($$anchor, () => "test", () => items[0], () => [42]);
}
