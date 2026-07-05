import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li><!></li>`), App[$.FILENAME], [[11, 4]]);
var root_1 = $.add_locations($.from_html(`<ul></ul>`), App[$.FILENAME], [[9, 2]]);
var root_2 = $.add_locations($.from_html(`<article><span class="name"> </span> <!></article>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let file = $.prop($$props, "file", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var article = root_2();
	var span = $.child(article);
	var text = $.child(span, true);
	$.reset(span);
	var node = $.sibling(span, 2);
	{
		var consequent = ($$anchor) => {
			var ul = root_1();
			$.add_svelte_meta(() => $.each(ul, 5, () => ($.deep_read_state(file()), $.untrack(() => file().children)), $.index, ($$anchor, child) => {
				var li = root();
				var node_1 = $.child(li);
				$.add_svelte_meta(() => App(node_1, { get file() {
					return $.get(child);
				} }), "component", App, 11, 8, { componentTag: "svelte:self" });
				$.reset(li);
				$.append($$anchor, li);
			}), "each", App, 10, 3);
			$.reset(ul);
			$.append($$anchor, ul);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.deep_read_state(file()), $.untrack(() => $.strict_equals(file().type, "folder"))) $$render(consequent);
		}), "if", App, 8, 1);
	}
	$.reset(article);
	$.template_effect(() => {
		$.set_class(article, 1, `file ${($.deep_read_state(file()), $.untrack(() => file().type)) ?? ""}`);
		$.set_text(text, ($.deep_read_state(file()), $.untrack(() => file().name)));
	});
	$.append($$anchor, article);
	return $.pop($$exports);
}
