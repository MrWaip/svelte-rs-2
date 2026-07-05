App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<a> </a>`), App[$.FILENAME], [[16, 12]]);
var root_1 = $.add_locations($.from_html(`<div> <!></div>`), App[$.FILENAME], [[13, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const groups = $.tag($.derived(() => {
		const groups = new Map();
		for (const x of $$props.data.schema) groups.set(x, [{
			name: x,
			href: x
		}]);
		return groups;
	}), "groups");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => $.get(groups), $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let group = () => $.get($$array)[0];
		group();
		let links = () => $.get($$array)[1];
		links();
		var div = root_1();
		var text = $.child(div);
		var node_1 = $.sibling(text);
		$.add_svelte_meta(() => $.each(node_1, 17, links, $.index, ($$anchor, $$item) => {
			let name = () => $.get($$item).name;
			name();
			let href = () => $.get($$item).href;
			href();
			var a = root();
			var text_1 = $.child(a, true);
			$.reset(a);
			$.template_effect(() => {
				$.set_attribute(a, "href", href());
				$.set_text(text_1, name());
			});
			$.append($$anchor, a);
		}), "each", App, 15, 8);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `${group() ?? ""} `));
		$.append($$anchor, div);
	}), "each", App, 12, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
