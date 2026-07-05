App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[14, 1]]);
var root_3 = $.add_locations($.from_html(`<!> <!> <!> <!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.prop($$props, "items", 19, () => []);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_3();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, items, ([id, name]) => id, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		let id = () => $.get($$array)[0];
		id();
		let name = () => $.get($$array)[1];
		name();
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, name()));
		$.append($$anchor, p);
	}), "each", App, 5, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 17, items, ({ id, name }) => id, ($$anchor, $$item) => {
		let id = () => $.get($$item).id;
		id();
		let name = () => $.get($$item).name;
		name();
		var p_1 = root_1();
		var text_1 = $.child(p_1, true);
		$.reset(p_1);
		$.template_effect(() => $.set_text(text_1, name()));
		$.append($$anchor, p_1);
	}), "each", App, 9, 0);
	var node_2 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => $.each(node_2, 19, items, ([id, name]) => id, ($$anchor, $$item, idx) => {
		var $$array_1 = $.derived(() => $.to_array($.get($$item), 2));
		let id = () => $.get($$array_1)[0];
		id();
		let name = () => $.get($$array_1)[1];
		name();
		var p_2 = root_2();
		var text_2 = $.child(p_2);
		$.reset(p_2);
		$.template_effect(() => $.set_text(text_2, `${$.get(idx) ?? ""}: ${name() ?? ""}`));
		$.append($$anchor, p_2);
	}), "each", App, 13, 0);
	var node_3 = $.sibling(node_2, 2);
	$.add_svelte_meta(() => $.each(node_3, 19, items, ([a, b, c]) => b.key, ($$anchor, $$item) => {
		var $$array_2 = $.derived(() => $.to_array($.get($$item), 3));
		let a = () => $.get($$array_2)[0];
		a();
		let b = () => $.get($$array_2)[1];
		b();
		let c = () => $.get($$array_2)[2];
		c();
	}), "each", App, 17, 0);
	var node_4 = $.sibling(node_3, 2);
	$.add_svelte_meta(() => $.each(node_4, 19, items, ({ a, b, c }) => b.key, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		a();
		let b = () => $.get($$item).b;
		b();
		let c = () => $.get($$item).c;
		c();
	}), "each", App, 20, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
