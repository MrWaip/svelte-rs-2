import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const items1 = $.mutable_source({});
	let data = [{
		id: 1,
		text: "a"
	}];
	var $$exports = {
		get items1() {
			return $.get(items1);
		},
		set items1($$value) {
			$.set(items1, $.proxy($$value));
		}
	};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => data, ({ id, text }) => id, ($$anchor, $$item) => {
		let id = () => $.get($$item).id;
		let text = () => $.get($$item).text;
		var div = root();
		var text_1 = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, id) => $.mutate(items1, $.get(items1)[id] = $$value), (id) => $.get(items1)?.[id], () => [id()]);
		$.template_effect(() => $.set_text(text_1, text()));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.bind_prop($$props, "items1", $.get(items1));
	return $.pop($$exports);
}
