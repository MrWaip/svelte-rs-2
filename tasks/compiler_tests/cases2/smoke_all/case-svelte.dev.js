App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
import { onMount } from "svelte";
const badge = $.wrap_snippet(App, function($$anchor, text = $.noop, variant = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root_1();
	let classes;
	var text_2 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => {
		classes = $.set_class(span, 1, "badge", null, classes, {
			primary: $.strict_equals(variant(), "primary"),
			secondary: $.strict_equals(variant(), "secondary")
		});
		$.set_text(text_2, text());
	});
	$.append($$anchor, span);
});
const card = $.wrap_snippet(App, function($$anchor, heading = $.noop, body = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root_2();
	var h3 = $.child(div);
	var text_3 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_4 = $.child(p, true);
	$.reset(p);
	var node = $.sibling(p, 2);
	$.add_svelte_meta(() => badge(node, () => "new", () => "primary"), "render", App, 60, 8);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text_3, heading());
		$.set_text(text_4, body());
	});
	$.append($$anchor, div);
});
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"title",
	"theme",
	"editable",
	"config",
	"multiplier"
]);
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[47, 4]]);
var root_1 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[51, 4]]);
var root_2 = $.add_locations($.from_html(`<div class="card"><h3> </h3> <p> </p> <!></div>`), App[$.FILENAME], [[
	57,
	4,
	[[58, 8], [59, 8]]
]]);
var root_3 = $.add_locations($.from_html(`<!> <div class="entry"> </div>`, 1), App[$.FILENAME], [[77, 12]]);
var root_4 = $.add_locations($.from_html(`<section><p> </p> <!></section>`), App[$.FILENAME], [[
	71,
	4,
	[[72, 8]]
]]);
var root_5 = $.add_locations($.from_html(`Title <p>Nothing here yet</p>`, 1), App[$.FILENAME], [[88, 12]]);
var root_6 = $.add_locations($.from_html(`<p>Nothing here yet</p> <!>`, 1), App[$.FILENAME], [[84, 8]]);
var root_7 = $.add_locations($.from_html(`<noscript></noscript> <p> </p>`, 1), App[$.FILENAME], [[92, 4], [93, 4]]);
var root_8 = $.add_locations($.from_html(`<span empty="">Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.</span>`), App[$.FILENAME], [[113, 12]]);
var root_9 = $.add_locations($.from_html(`<h1>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.</h1>`), App[$.FILENAME], [[125, 16]]);
var root_10 = $.add_locations($.from_html(`<h2>EMPTY</h2>`), App[$.FILENAME], [[133, 16]]);
var root_11 = $.add_locations($.from_html(`<div><input/></div> <!>`, 1), App[$.FILENAME], [[
	120,
	12,
	[[121, 16]]
]]);
var root_12 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[139, 8]]);
var root_13 = $.add_locations($.from_html(`<header><h1> </h1> <input/> <button> </button></header> <!> <!> <div> <p> </p> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. <!></div> <!> <input/> <!> <!></div>`, 1), App[$.FILENAME], [[
	64,
	0,
	[
		[65, 4],
		[66, 4],
		[67, 4]
	]
], [
	98,
	0,
	[
		[100, 4],
		[101, 4],
		[142, 4]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor, item = $.noop) {
		$.validate_snippet_args(...arguments);
		var li = root();
		var text_1 = $.child(li);
		$.reset(li);
		$.template_effect(() => $.set_text(text_1, `${item() ?? ""} — ${$.get(count) ?? ""}`));
		$.append($$anchor, li);
	});
	let theme = $.prop($$props, "theme", 3, "light"), config = $.prop($$props, "config", 27, () => $.tag_proxy($.proxy({}), "config")), multiplier = $.prop($$props, "multiplier", 3, 2), extras = $.rest_props($$props, rest_excludes, "extras");
	let count = $.tag($.state(0), "count");
	let query = $.tag($.state(""), "query");
	let state = $.tag($.state(""), "state");
	let counter = $.tag($.state(0), "counter");
	let items = [
		"Задачи",
		"Settings",
		"🌞 Profile"
	];
	$.set(counter, 10);
	$.set(count, $.get(count) + 1);
	let doubled = $.tag($.derived(() => $.get(count) * multiplier()), "doubled");
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", "Title:", $$props.title, "Count:", $.get(count)));
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
		...$.legacy_api(),
		get VERSION() {
			return VERSION;
		},
		get APP_VERSION() {
			return APP_VERSION;
		},
		get reset() {
			return reset;
		},
		get formatTitle() {
			return formatTitle;
		}
	};
	var fragment = root_13();
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
			$.add_svelte_meta(() => $.each(node_2, 17, () => items, $.index, ($$anchor, item) => {
				var fragment_1 = root_3();
				var node_3 = $.first_child(fragment_1);
				$.add_svelte_meta(() => row(node_3, () => $.get(item)), "render", App, 75, 12);
				var div_1 = $.sibling(node_3, 2);
				var text_8 = $.child(div_1, true);
				$.reset(div_1);
				$.template_effect(() => {
					$.set_attribute(div_1, "data-q", `q: ${$.get(query) ?? ""}`);
					$.set_text(text_8, $.get(item));
				});
				$.append($$anchor, fragment_1);
			}), "each", App, 74, 8);
			$.reset(section);
			$.template_effect(() => $.set_text(text_7, `Результат: ${$.get(count) ?? ""} for ${$.get(query) ?? ""}`));
			$.append($$anchor, section);
		};
		var consequent_1 = ($$anchor) => {
			$.add_svelte_meta(() => Panel($$anchor, {
				label: "empty",
				get count() {
					return $.get(count);
				},
				children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
					var fragment_3 = root_6();
					var node_4 = $.sibling($.first_child(fragment_3), 2);
					$.add_svelte_meta(() => Panel(node_4, {
						label: "empty",
						get count() {
							return $.get(count);
						},
						children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
							$.next();
							var fragment_4 = root_5();
							$.next();
							$.append($$anchor, fragment_4);
						}),
						$$slots: { default: true }
					}), "component", App, 86, 8, { componentTag: "Panel" });
					$.append($$anchor, fragment_3);
				}),
				$$slots: { default: true }
			}), "component", App, 83, 4, { componentTag: "Panel" });
		};
		var alternate = ($$anchor) => {
			var fragment_5 = root_7();
			var p_2 = $.sibling($.first_child(fragment_5), 2);
			var text_9 = $.child(p_2, true);
			$.reset(p_2);
			$.template_effect(() => $.set_text(text_9, $.set(count, 0)));
			$.append($$anchor, fragment_5);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if ($.get(count) > 0) $$render(consequent);
			else if ($$props.editable) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		}), "if", App, 70, 0);
	}
	var node_5 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => Panel(node_5, {
		get count() {
			return $.get(count);
		},
		get label() {
			return $$props.title;
		}
	}), "component", App, 96, 0, { componentTag: "Panel" });
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
			var span_1 = root_8();
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
					var h1_1 = root_9();
					$.template_effect(() => $.set_attribute(h1_1, "state", $.get(state)));
					$.append($$anchor, h1_1);
				};
				var consequent_4 = ($$anchor) => {
					var text_12 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_12);
				};
				var alternate_1 = ($$anchor) => {
					var h2 = root_10();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_7, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_3);
					else if ($.equals($.get(counter), 100)) $$render(consequent_4, 1);
					else $$render(alternate_1, -1);
				}), "if", App, 124, 12);
			}
			$.template_effect(() => {
				$.set_attribute(input_1, "title", $$props.title);
				$.set_attribute(input_1, "state", $.get(state));
				$.set_value(input_1, $.get(count));
			});
			$.append($$anchor, fragment_6);
		};
		$.add_svelte_meta(() => $.if(node_6, ($$render) => {
			if ($.get(state)) $$render(consequent_2);
			else $$render(alternate_2, -1);
		}), "if", App, 112, 8);
	}
	$.reset(div_3);
	var node_8 = $.sibling(div_3, 2);
	$.add_svelte_meta(() => $.each(node_8, 17, () => items, $.index, ($$anchor, item) => {
		var p_4 = root_12();
		$.attribute_effect(p_4, () => ({
			...extras,
			"data-index": "chunk-0"
		}));
		var text_13 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_13, $.get(item)));
		$.append($$anchor, p_4);
	}), "each", App, 138, 4);
	var input_2 = $.sibling(node_8, 2);
	$.remove_input_defaults(input_2);
	var node_9 = $.sibling(input_2, 2);
	$.add_svelte_meta(() => badge(node_9, () => "chunk-0", () => "secondary"), "render", App, 144, 4);
	var node_10 = $.sibling(node_9, 2);
	$.add_svelte_meta(() => card(node_10, () => $$props.title, () => "Content for chunk 0"), "render", App, 145, 4);
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
	$.bind_value(input, function get() {
		return $.get(query);
	}, function set($$value) {
		$.set(query, $$value);
	});
	$.delegated("click", button, increment);
	$.bind_value(input_2, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
