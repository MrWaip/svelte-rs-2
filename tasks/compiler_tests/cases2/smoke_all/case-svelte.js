import * as $ from "svelte/internal/client";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
import { onMount } from "svelte";
const badge = ($$anchor, text = $.noop, variant = $.noop) => {
	var span = root_2();
	let classes;
	var text_2 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => {
		classes = $.set_class(span, 1, "badge", null, classes, {
			primary: variant() === "primary",
			secondary: variant() === "secondary"
		});
		$.set_text(text_2, text());
	});
	$.append($$anchor, span);
};
const card = ($$anchor, heading = $.noop, body = $.noop) => {
	var div = root_3();
	var h3 = $.child(div);
	var text_3 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_4 = $.child(p, true);
	$.reset(p);
	var node = $.sibling(p, 2);
	badge(node, () => "new", () => "primary");
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text_3, heading());
		$.set_text(text_4, body());
	});
	$.append($$anchor, div);
};
var root_1 = $.from_html(`<li> </li>`);
var root_2 = $.from_html(`<span> </span>`);
var root_3 = $.from_html(`<div class="card"><h3> </h3> <p> </p> <!></div>`);
var root_5 = $.from_html(`<!> <div class="entry"> </div>`, 1);
var root_4 = $.from_html(`<section><p> </p> <!></section>`);
var root_8 = $.from_html(`Title <p>Nothing here yet</p>`, 1);
var root_7 = $.from_html(`<p>Nothing here yet</p> <!>`, 1);
var root_9 = $.from_html(`<noscript></noscript> <p> </p>`, 1);
var root_10 = $.from_html(`<span empty="">Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.</span>`);
var root_12 = $.from_html(`<h1>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.</h1>`);
var root_14 = $.from_html(`<h2>EMPTY</h2>`);
var root_11 = $.from_html(`<div><input/></div> <!>`, 1);
var root_15 = $.from_html(`<p> </p>`);
var root = $.from_html(`<header><h1> </h1> <input/> <button> </button></header> <!> <!> <div> <p> </p> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. <!></div> <!> <input/> <!> <!></div>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const row = ($$anchor, item = $.noop) => {
		var li = root_1();
		var text_1 = $.child(li);
		$.reset(li);
		$.template_effect(() => $.set_text(text_1, `${item() ?? ""} — ${$.get(count) ?? ""}`));
		$.append($$anchor, li);
	};
	let theme = $.prop($$props, "theme", 3, "light"), config = $.prop($$props, "config", 27, () => $.proxy({})), multiplier = $.prop($$props, "multiplier", 3, 2), extras = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"title",
		"theme",
		"editable",
		"config",
		"multiplier"
	]);
	let count = $.state(0);
	let query = $.state("");
	let state = $.state("");
	let counter = $.state(0);
	let items = [
		"Задачи",
		"Settings",
		"🌞 Profile"
	];
	$.set(counter, 10);
	$.set(count, $.get(count) + 1);
	let doubled = $.derived(() => $.get(count) * multiplier());
	$.user_effect(() => {
		console.log("Title:", $$props.title, "Count:", $.get(count));
	});
	const VERSION = "2.0";
	const APP_VERSION = "1.0.0";
	function reset() {
		$.set(count, 0);
	}
	function formatTitle(prefix) {
		return prefix + ": " + $$props.title;
	}
	function increment() {
		$.update(count);
	}
	var $$exports = {
		VERSION,
		APP_VERSION,
		reset,
		formatTitle
	};
	var fragment = root();
	var header = $.first_child(fragment);
	$.attribute_effect(header, () => ({
		id: "top",
		"data-theme": theme(),
		title: `Dashboard: ${$$props.title ?? ""}`,
		...extras
	}));
	var h1 = $.child(header);
	var text_5 = $.child(h1);
	$.reset(h1);
	var input = $.sibling(h1, 2);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	var text_6 = $.child(button, true);
	$.reset(button);
	$.reset(header);
	var node_1 = $.sibling(header, 2);
	{
		var consequent = ($$anchor) => {
			var section = root_4();
			var p_1 = $.child(section);
			var text_7 = $.child(p_1);
			$.reset(p_1);
			var node_2 = $.sibling(p_1, 2);
			$.each(node_2, 17, () => items, $.index, ($$anchor, item) => {
				var fragment_1 = root_5();
				var node_3 = $.first_child(fragment_1);
				row(node_3, () => $.get(item));
				var div_1 = $.sibling(node_3, 2);
				var text_8 = $.child(div_1, true);
				$.reset(div_1);
				$.template_effect(() => {
					$.set_attribute(div_1, "data-q", `q: ${$.get(query) ?? ""}`);
					$.set_text(text_8, $.get(item));
				});
				$.append($$anchor, fragment_1);
			});
			$.reset(section);
			$.template_effect(() => $.set_text(text_7, `Результат: ${$.get(count) ?? ""} for ${$.get(query) ?? ""}`));
			$.append($$anchor, section);
		};
		var consequent_1 = ($$anchor) => {
			Panel($$anchor, {
				label: "empty",
				get count() {
					return $.get(count);
				},
				children: ($$anchor, $$slotProps) => {
					var fragment_3 = root_7();
					var node_4 = $.sibling($.first_child(fragment_3), 2);
					Panel(node_4, {
						label: "empty",
						get count() {
							return $.get(count);
						},
						children: ($$anchor, $$slotProps) => {
							$.next();
							var fragment_4 = root_8();
							$.next();
							$.append($$anchor, fragment_4);
						},
						$$slots: { default: true }
					});
					$.append($$anchor, fragment_3);
				},
				$$slots: { default: true }
			});
		};
		var alternate = ($$anchor) => {
			var fragment_5 = root_9();
			var p_2 = $.sibling($.first_child(fragment_5), 2);
			var text_9 = $.child(p_2, true);
			$.reset(p_2);
			$.template_effect(() => $.set_text(text_9, $.set(count, 0)));
			$.append($$anchor, fragment_5);
		};
		$.if(node_1, ($$render) => {
			if ($.get(count) > 0) $$render(consequent);
			else if ($$props.editable) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	var node_5 = $.sibling(node_1, 2);
	Panel(node_5, {
		get count() {
			return $.get(count);
		},
		get label() {
			return $$props.title;
		}
	});
	var div_2 = $.sibling(node_5, 2);
	var text_10 = $.child(div_2);
	var p_3 = $.sibling(text_10);
	var text_11 = $.child(p_3);
	$.reset(p_3);
	var div_3 = $.sibling(p_3, 2);
	let classes_1;
	var node_6 = $.sibling($.child(div_3));
	{
		var consequent_2 = ($$anchor) => {
			var span_1 = root_10();
			$.template_effect(() => {
				$.set_attribute(span_1, "title", `${$$props.title ?? ""}: ${$.get(doubled) ?? ""}`);
				$.set_attribute(span_1, "state", $.get(state));
				$.set_attribute(span_1, "counter", $.get(counter));
				$.set_attribute(span_1, "count", $.get(count));
			});
			$.append($$anchor, span_1);
		};
		var alternate_2 = ($$anchor) => {
			var fragment_6 = root_11();
			var div_4 = $.first_child(fragment_6);
			var input_1 = $.child(div_4);
			$.remove_input_defaults(input_1);
			$.reset(div_4);
			var node_7 = $.sibling(div_4, 2);
			{
				var consequent_3 = ($$anchor) => {
					var h1_1 = root_12();
					$.template_effect(() => $.set_attribute(h1_1, "state", $.get(state)));
					$.append($$anchor, h1_1);
				};
				var consequent_4 = ($$anchor) => {
					var text_12 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_12);
				};
				var alternate_1 = ($$anchor) => {
					var h2 = root_14();
					$.append($$anchor, h2);
				};
				$.if(node_7, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_3);
					else if ($.get(counter) == 100) $$render(consequent_4, 1);
					else $$render(alternate_1, -1);
				});
			}
			$.template_effect(() => {
				$.set_attribute(input_1, "title", $$props.title);
				$.set_attribute(input_1, "state", $.get(state));
				$.set_value(input_1, $.get(count));
			});
			$.append($$anchor, fragment_6);
		};
		$.if(node_6, ($$render) => {
			if ($.get(state)) $$render(consequent_2);
			else $$render(alternate_2, -1);
		});
	}
	$.reset(div_3);
	var node_8 = $.sibling(div_3, 2);
	$.each(node_8, 17, () => items, $.index, ($$anchor, item) => {
		var p_4 = root_15();
		$.attribute_effect(p_4, () => ({
			...extras,
			"data-index": "chunk-0"
		}));
		var text_13 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_13, $.get(item)));
		$.append($$anchor, p_4);
	});
	var input_2 = $.sibling(node_8, 2);
	$.remove_input_defaults(input_2);
	var node_9 = $.sibling(input_2, 2);
	badge(node_9, () => "chunk-0", () => "secondary");
	var node_10 = $.sibling(node_9, 2);
	card(node_10, () => $$props.title, () => "Content for chunk 0");
	$.reset(div_2);
	$.template_effect(() => {
		$.set_text(text_5, `${$$props.title ?? ""} 🚀`);
		$.set_text(text_6, $.get(count));
		$.set_text(text_10, `Chunk 0: Lorem ${$.get(state) ?? ""} + ${$.get(state) ?? ""} = Ipsum; `);
		$.set_text(text_11, `Props: title=${$$props.title ?? ""}, count=${$.get(count) ?? ""}, doubled=${$.get(doubled) ?? ""}`);
		classes_1 = $.set_class(div_3, 1, "", null, classes_1, {
			state: $.get(state),
			staticly: true,
			invinsible,
			reactive: $.get(counter)
		});
	});
	$.bind_value(input, () => $.get(query), ($$value) => $.set(query, $$value));
	$.delegated("click", button, increment);
	$.bind_value(input_2, () => $.get(state), ($$value) => $.set(state, $$value));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
