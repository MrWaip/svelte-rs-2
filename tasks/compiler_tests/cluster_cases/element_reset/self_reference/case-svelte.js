import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<li><!></li>`);
var root_1 = $.from_html(`<ul></ul>`);
var root_2 = $.from_html(`<article><span class="name"> </span> <!></article>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let file = $.prop($$props, "file", 8);
	$.init();
	var article = root_2();
	var span = $.child(article);
	var text = $.child(span, true);
	$.reset(span);
	var node = $.sibling(span, 2);
	{
		var consequent = ($$anchor) => {
			var ul = root_1();
			$.each(ul, 5, () => ($.deep_read_state(file()), $.untrack(() => file().children)), $.index, ($$anchor, child) => {
				var li = root();
				var node_1 = $.child(li);
				App(node_1, { get file() {
					return $.get(child);
				} });
				$.reset(li);
				$.append($$anchor, li);
			});
			$.reset(ul);
			$.append($$anchor, ul);
		};
		$.if(node, ($$render) => {
			if ($.deep_read_state(file()), $.untrack(() => file().type === "folder")) $$render(consequent);
		});
	}
	$.reset(article);
	$.template_effect(() => {
		$.set_class(article, 1, `file ${($.deep_read_state(file()), $.untrack(() => file().type)) ?? ""}`);
		$.set_text(text, ($.deep_read_state(file()), $.untrack(() => file().name)));
	});
	$.append($$anchor, article);
	$.pop();
}
