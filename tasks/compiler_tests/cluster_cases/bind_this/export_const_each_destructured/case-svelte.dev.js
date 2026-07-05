import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const items1 = $.tag($.mutable_source({}), "items1");
	let data = [{
		id: 1,
		text: "a"
	}];
	var $$exports = {
		...$.legacy_api(),
		get items1() {
			return $.get(items1);
		},
		set items1($$value) {
			$.set(items1, $.proxy($$value));
		}
	};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => data, ({ id, text }) => id, ($$anchor, $$item) => {
		let id = () => $.get($$item).id;
		id();
		let text = () => $.get($$item).text;
		text();
		var div = root();
		var text_1 = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, id) => $.mutate(items1, $.get(items1)[id] = $$value), (id) => $.get(items1)?.[id], () => [id()]);
		$.template_effect(() => $.set_text(text_1, text()));
		$.append($$anchor, div);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	$.bind_prop($$props, "items1", $.get(items1));
	return $.pop($$exports);
}
