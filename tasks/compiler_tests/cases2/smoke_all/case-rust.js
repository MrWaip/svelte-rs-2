import * as $ from "svelte/internal/client";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
const row = ($$anchor, item = $.noop) => {
	var li = root_6();
	var text_5 = $.child(li);
	$.reset(li);
	$.template_effect(() => $.set_text(text_5, `${item() ?? ""} — ${$.get(count) ?? ""}`));
	$.append($$anchor, li);
};
var root_2 = $.from_html(`<!> <div class="entry"> </div>`, 1);
var root_1 = $.from_html(`<section><p> </p> <!></section>`);
var root_4 = $.from_html(`<p>Nothing here yet</p>`);
var root_5 = $.from_html(`<noscript></noscript> <p> </p>`, 1);
var root_6 = $.from_html(`<li> </li>`);
var root = $.from_html(`<header><h1> </h1> <input/> <button> </button></header> <!> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let theme = $.prop($$props, "theme", 3, "light"), extras = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"title",
		"theme",
		"editable"
	]);
	let count = $.state(0);
	let query = $.state("");
	let items = [
		"Задачи",
		"Settings",
		"🌞 Profile"
	];
	const VERSION = "2.0";
	function reset() {
		$.set(count, 0);
	}
	function increment() {
		$.update(count);
	}
	$.set(count, $.get(count) + 1);
	var $$exports = {
		VERSION,
		reset
	};
	var fragment = root();
	var header = $.first_child(fragment);
	$.attribute_effect(header, () => ({
		id: "top",
		data-theme: theme(),
		title: `Dashboard: ${$$props.title ?? ""}`,
		...extras()
	}));
	var h1 = $.child(header);
	var text = $.child(h1);
	$.reset(h1);
	var input = $.sibling(h1, 2);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	$.set_attribute(button, "onclick", increment);
	var text_1 = $.child(button, true);
	$.reset(button);
	$.reset(header);
	var node = $.sibling(header, 2);
	{
		var consequent = ($$anchor) => {
			var section = root_1();
			var p = $.child(section);
			var text_2 = $.child(p);
			$.reset(p);
			var node_1 = $.sibling(p, 2);
			$.each(node_1, 16, () => items, $.index, ($$anchor, item) => {
				var fragment_1 = root_2();
				var node_2 = $.first_child(fragment_1);
				row(node_2, () => item);
				var div = $.sibling(node_2, 2);
				var text_3 = $.child(div, true);
				$.reset(div);
				$.template_effect(() => {
					$.set_attribute(div, "data-q", `q: ${$.get(query) ?? ""}`);
					$.set_text(text_3, item);
				});
				$.append($$anchor, fragment_1);
			});
			$.reset(section);
			$.template_effect(() => $.set_text(text_2, `Результат: ${$.get(count) ?? ""} for `));
			$.append($$anchor, section);
		};
		var consequent_1 = ($$anchor) => Panel($$anchor, {
			label: "empty",
			get count() {
				return $.get(count);
			},
			children: ($$anchor, $$slotProps) => {
				var p_1 = root_4();
				$.append($$anchor, p_1);
			},
			$$slots: { default: true }
		});
		var alternate = ($$anchor) => {
			var fragment_2 = root_5();
			var p_2 = $.sibling($.first_child(fragment_2), 2);
			var text_4 = $.child(p_2, true);
			$.reset(p_2);
			$.template_effect(() => $.set_text(text_4, $.set(count, 0)));
			$.append($$anchor, fragment_2);
		};
		$.if(node, ($$render) => {
			if ($.get(count) > 0) $$render(consequent);
			else if ($$props.editable) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	var node_3 = $.sibling(node, 2);
	Panel(node_3, {
		get count() {
			return $.get(count);
		},
		get label() {
			return $$props.title;
		}
	});
	$.template_effect(() => {
		$.set_text(text, `${$$props.title ?? ""} 🚀`);
		$.set_text(text_1, $.get(count));
	});
	$.bind_value(input, () => $.get(query), ($$value) => $.set(query, $$value));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
