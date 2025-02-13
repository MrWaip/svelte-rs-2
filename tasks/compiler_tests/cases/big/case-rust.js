import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
var root_1 = $.template(`<span empty="">text</span>`);
var root_3 = $.template(`<h1>text</h1>`);
var root_6 = $.template(`<h2>EMPTY</h2>`);
var root_2 = $.template(`<div><input></div> <!>`, 1);
var root = $.template(`<div> <div>sequence <!></div></div>`);
export default function App($$anchor) {
	let state = "";
	let counter = $.state(0);
	$.set(counter, 10);
	var div = root();
	var text = $.child(div);
	var div_1 = $.sibling(text);
	$.toggle_class(div_1, "staticly", true);
	$.toggle_class(div_1, "invinsible", invinsible);
	var node = $.sibling($.child(div_1));
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			$.template_effect(() => {
				$.set_attribute(span, "title", `${state ?? ""}__________${state ?? ""}`);
				$.set_attribute(span, "state", state);
				$.set_attribute(span, "counter", $.get(counter));
			});
			$.append($$anchor, span);
		};
		var alternate_2 = ($$anchor) => {
			var fragment = root_2();
			var div_2 = $.first_child(fragment);
			var input = $.child(div_2);
			$.set_attribute(input, "title", title);
			$.reset(div_2);
			var node_1 = $.sibling(div_2, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_3();
					$.template_effect(() => $.set_attribute(h1, "state", state));
					$.append($$anchor, h1);
				};
				var alternate_1 = ($$anchor) => {
					var fragment_1 = $.comment();
					var node_2 = $.first_child(fragment_1);
					{
						var consequent_2 = ($$anchor) => {
							var text_1 = $.text("text");
							$.append($$anchor, text_1);
						};
						var alternate = ($$anchor) => {
							var h2 = root_6();
							$.append($$anchor, h2);
						};
						$.if(node_2, ($$render) => {
							if ($.get(counter) == 100) $$render(consequent_2);
else $$render(alternate, false);
						}, true);
					}
					$.append($$anchor, fragment_1);
				};
				$.if(node_1, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_1);
else $$render(alternate_1, false);
				});
			}
			$.template_effect(() => $.set_attribute(input, "state", state));
			$.append($$anchor, fragment);
		};
		$.if(node, ($$render) => {
			if (state) $$render(consequent);
else $$render(alternate_2, false);
		});
	}
	$.reset(div_1);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text, `Lorem ${state ?? ""} + ${state ?? ""} = Ipsum; `);
		$.toggle_class(div_1, "state", state);
		$.toggle_class(div_1, "reactive", $.get(counter));
	});
	$.append($$anchor, div);
}
