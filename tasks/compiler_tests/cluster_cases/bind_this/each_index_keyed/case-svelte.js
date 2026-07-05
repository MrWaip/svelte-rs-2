import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let data = $.prop($$props, "data", 24, () => [{ id: "1" }]);
	let refs = $.prop($$props, "refs", 28, () => []);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 3, data, ({ id }) => id, ($$anchor, $$item, index) => {
		let id = () => $.get($$item).id;
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, index) => refs(refs()[index] = $$value, true), (index) => refs()?.[index], () => [$.get(index)]);
		$.template_effect(() => $.set_text(text, id()));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
}
