import * as $ from "svelte/internal/client";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
import { onMount } from "svelte";
const badge = ($$anchor, text = $.noop, variant = $.noop) => {
	var span_1 = root_14();
	let classes;
	$.template_effect(() => classes = $.set_class(span_1, 1, "", null, classes, {
		primary: variant() === "primary",
		secondary: variant() === "secondary"
	}));
	var text_10 = $.child(span_1, true);
	$.reset(span_1);
	$.template_effect(() => $.set_text(text_10, text()));
	$.append($$anchor, span_1);
};
const card = ($$anchor, heading = $.noop, body = $.noop) => {
	var div_4 = root_15();
	var h3 = $.child(div_4);
	var text_11 = $.child(h3, true);
	$.reset(h3);
	var p_4 = $.sibling(h3, 2);
	var text_12 = $.child(p_4, true);
	$.reset(p_4);
	var node_10 = $.sibling(p_4, 2);
	badge(node_10, () => "new", () => "primary");
	$.reset(div_4);
	$.template_effect(() => {
		$.set_text(text_11, heading());
		$.set_text(text_12, body());
	});
	$.append($$anchor, div_4);
};
var root_1 = $.from_html(`<li> </li>`);
var root_3 = $.from_html(`<!> <div class="entry"> </div>`, 1);
var root_2 = $.from_html(`<section><p> </p> <!></section>`);
var root_6 = $.from_html(`Title <p>Nothing here yet</p>`, 1);
var root_5 = $.from_html(`<p>Nothing here yet</p> <!>`, 1);
var root_7 = $.from_html(`<noscript></noscript> <p> </p>`, 1);
var root_8 = $.from_html(`<span empty="">Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.</span>`);
var root_10 = $.from_html(`<h1>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.</h1>`);
var root_12 = $.from_html(`<h2>EMPTY</h2>`);
var root_9 = $.from_html(`<div><input/></div> <!>`, 1);
var root_13 = $.from_html(`<p> </p>`);
var root_14 = $.from_html(`<span class="badge"> </span>`);
var root_15 = $.from_html(`<div class="card"><h3> </h3> <p> </p> <!></div>`);
var root = $.from_html(`<header><h1> </h1> <input/> <button> </button></header> <!> <!> <div> <p> </p> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. <!></div> <!> <input/> <!> <!></div>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const row = ($$anchor, item = $.noop) => {
		var li = root_1();
		var text = $.child(li);
		$.reset(li);
		$.template_effect(() => $.set_text(text, `${item() ?? ""} — ${$.get(count) ?? ""}`));
		$.append($$anchor, li);
	};
	let theme = $.prop($$props, "theme", 3, "light"), config = $.prop($$props, "config", 27, () => ({})), multiplier = $.prop($$props, "multiplier", 3, 2), extras = $.rest_props($$props, [
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
	$effect(() => {
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
	var text_1 = $.child(h1);
	$.reset(h1);
	var input = $.sibling(h1, 2);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	var text_2 = $.child(button, true);
	$.reset(button);
	$.reset(header);
	var node = $.sibling(header, 2);
	{
		var consequent = ($$anchor) => {
			var section = root_2();
			var p = $.child(section);
			var text_3 = $.child(p);
			$.reset(p);
			var node_1 = $.sibling(p, 2);
			$.each(node_1, 17, () => items, $.index, ($$anchor, item) => {
				var fragment_1 = root_3();
				var node_2 = $.first_child(fragment_1);
				row(node_2, () => $.get(item));
				var div = $.sibling(node_2, 2);
				var text_4 = $.child(div, true);
				$.reset(div);
				$.template_effect(() => {
					$.set_attribute(div, "data-q", `q: ${$.get(query) ?? ""}`);
					$.set_text(text_4, $.get(item));
				});
				$.append($$anchor, fragment_1);
			});
			$.reset(section);
			$.template_effect(() => $.set_text(text_3, `Результат: ${$.get(count) ?? ""} for ${$.get(query) ?? ""}`));
			$.append($$anchor, section);
		};
		var consequent_1 = ($$anchor) => {
			Panel($$anchor, {
				label: "empty",
				get count() {
					return $.get(count);
				},
				children: ($$anchor, $$slotProps) => {
					var fragment_3 = root_5();
					var node_3 = $.sibling($.first_child(fragment_3), 2);
					Panel(node_3, {
						label: "empty",
						get count() {
							return $.get(count);
						},
						children: ($$anchor, $$slotProps) => {
							$.next();
							var fragment_4 = root_6();
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
			var fragment_5 = root_7();
			var p_1 = $.sibling($.first_child(fragment_5), 2);
			var text_5 = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text_5, $.set(count, 0)));
			$.append($$anchor, fragment_5);
		};
		$.if(node, ($$render) => {
			if ($.get(count) > 0) $$render(consequent);
			else if ($$props.editable) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	var node_4 = $.sibling(node, 2);
	Panel(node_4, {
		get count() {
			return $.get(count);
		},
		get label() {
			return $$props.title;
		}
	});
	var div_1 = $.sibling(node_4, 2);
	var text_6 = $.child(div_1);
	var p_2 = $.sibling(text_6);
	var text_7 = $.child(p_2);
	$.reset(p_2);
	var div_2 = $.sibling(p_2, 2);
	let classes;
	$.template_effect(() => classes = $.set_class(div_2, 1, "", null, classes, {
		state: $.get(state),
		staticly: true,
		invinsible,
		reactive: $.get(counter)
	}));
	var node_5 = $.sibling($.child(div_2));
	{
		var consequent_2 = ($$anchor) => {
			var span = root_8();
			$.set_attribute(span, "title", `${$$props.title ?? ""}: ${$.get(doubled) ?? ""}`);
			$.template_effect(() => {
				$.set_attribute(span, "state", $.get(state));
				$.set_attribute(span, "counter", $.get(counter));
				$.set_attribute(span, "count", $.get(count));
			});
			$.append($$anchor, span);
		};
		var alternate_2 = ($$anchor) => {
			var fragment_6 = root_9();
			var div_3 = $.first_child(fragment_6);
			var input_1 = $.child(div_3);
			$.set_attribute(input_1, "title", $$props.title);
			$.reset(div_3);
			var node_6 = $.sibling(div_3, 2);
			{
				var consequent_3 = ($$anchor) => {
					var h1_1 = root_10();
					$.template_effect(() => $.set_attribute(h1_1, "state", $.get(state)));
					$.append($$anchor, h1_1);
				};
				var consequent_4 = ($$anchor) => {
					var text_8 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_8);
				};
				var alternate_1 = ($$anchor) => {
					var h2 = root_12();
					$.append($$anchor, h2);
				};
				$.if(node_6, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_3);
					else if ($.get(counter) == 100) $$render(consequent_4, 1);
					else $$render(alternate_1, -1);
				});
			}
			$.template_effect(() => {
				$.set_attribute(input_1, "state", $.get(state));
				$.set_attribute(input_1, "value", $.get(count));
			});
			$.append($$anchor, fragment_6);
		};
		$.if(node_5, ($$render) => {
			if ($.get(state)) $$render(consequent_2);
			else $$render(alternate_2, -1);
		});
	}
	$.reset(div_2);
	var node_7 = $.sibling(div_2, 2);
	$.each(node_7, 17, () => items, $.index, ($$anchor, item) => {
		var p_3 = root_13();
		$.attribute_effect(p_3, () => ({
			...extras,
			"data-index": "chunk-0"
		}));
		var text_9 = $.child(p_3, true);
		$.reset(p_3);
		$.template_effect(() => $.set_text(text_9, $.get(item)));
		$.append($$anchor, p_3);
	});
	var input_2 = $.sibling(node_7, 2);
	$.remove_input_defaults(input_2);
	var node_8 = $.sibling(input_2, 2);
	badge(node_8, () => "chunk-0", () => "secondary");
	var node_9 = $.sibling(node_8, 2);
	card(node_9, () => $$props.title, () => "Content for chunk 0");
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text_1, `${$$props.title ?? ""} 🚀`);
		$.set_text(text_2, $.get(count));
		$.set_text(text_6, `Chunk 0: Lorem ${$.get(state) ?? ""} + ${$.get(state) ?? ""} = Ipsum; `);
		$.set_text(text_7, `Props: title=${$$props.title ?? ""}, count=${$.get(count) ?? ""}, doubled=${$.get(doubled) ?? ""}`);
	});
	$.bind_value(input, () => $.get(query), ($$value) => $.set(query, $$value));
	$.delegated("click", button, increment);
	$.bind_value(input_2, () => $.get(state), ($$value) => $.set(state, $$value));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
