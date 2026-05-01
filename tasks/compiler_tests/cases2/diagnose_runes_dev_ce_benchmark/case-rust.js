App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
const badge = $.wrap_snippet(App, function($$anchor, text = $.noop, variant = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root_2();
	let classes;
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => {
		classes = $.set_class(span, 1, "badge svelte-13nvtxg", null, classes, {
			primary: variant() === "primary",
			secondary: variant() === "secondary"
		});
		$.set_text(text_1, text());
	});
	$.append($$anchor, span);
});
const card = $.wrap_snippet(App, function($$anchor, heading = $.noop, body = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root_3();
	var h3 = $.child(div);
	var text_2 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_3 = $.child(p, true);
	$.reset(p);
	var node_1 = $.sibling(p, 2);
	$.add_svelte_meta(() => badge(node_1, () => "new", () => "primary"), "render", App, 174, 8);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text_2, heading());
		$.set_text(text_3, body());
	});
	$.append($$anchor, div);
});
export const BENCHMARK_KIND = "compiler";
export const MODULE_SCALE = 3;
export function moduleLabel(name) {
	return `${BENCHMARK_KIND}:${name}`;
}
var root_1 = $.add_locations($.from_html(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/> <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`, 1), App[$.FILENAME], [
	[155, 4],
	[156, 4],
	[157, 4]
]);
var root_2 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[165, 4]]);
var root_3 = $.add_locations($.from_html(`<div class="card svelte-13nvtxg"><h3 class="svelte-13nvtxg"> </h3> <p class="svelte-13nvtxg"> </p> <!></div>`), App[$.FILENAME], [[
	171,
	4,
	[[172, 8], [173, 8]]
]]);
var root_5 = $.add_locations($.from_html(`<span class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[182, 12]]);
var root_4 = $.add_locations($.from_html(`<section class="summary svelte-13nvtxg"><h4 class="svelte-13nvtxg"> </h4> <!></section>`), App[$.FILENAME], [[
	179,
	4,
	[[180, 8]]
]]);
var root_6 = $.add_locations($.from_html(`<span empty="" class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[216, 12]]);
var root_8 = $.add_locations($.from_html(`<h1 class="svelte-13nvtxg">Lorem ipsum dolor sit amet. Chunk 0.</h1>`), App[$.FILENAME], [[225, 16]]);
var root_10 = $.add_locations($.from_html(`<h2 class="svelte-13nvtxg">EMPTY</h2>`), App[$.FILENAME], [[231, 16]]);
var root_7 = $.add_locations($.from_html(`<div class="svelte-13nvtxg"><input class="svelte-13nvtxg"/></div> <!>`, 1), App[$.FILENAME], [[
	220,
	12,
	[[221, 16]]
]]);
var root_11 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[237, 8]]);
var root_12 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[242, 8]]);
var root_13 = $.add_locations($.from_html(`<span class="item-less svelte-13nvtxg">Repeated shell chunk 0</span>`), App[$.FILENAME], [[246, 8]]);
var root_14 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[252, 8]]);
var root_15 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[254, 8]]);
var root_16 = $.add_locations($.from_html(`<p class="svelte-13nvtxg">Loading chunk 0...</p>`), App[$.FILENAME], [[250, 8]]);
var root_17 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[258, 8]]);
var root_19 = $.add_locations($.from_html(`<strong class="svelte-13nvtxg"> </strong>`), App[$.FILENAME], [[280, 8]]);
var root_20 = $.add_locations($.from_html(`<div slot="footer" class="svelte-13nvtxg"> </div>`), App[$.FILENAME], []);
var root_21 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[295, 12]]);
var root_22 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[293, 8]]);
var root = $.add_locations($.from_html(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0"> <p class="svelte-13nvtxg"> </p> <p class="svelte-13nvtxg"> </p> <!> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. <!></div> <!> <!> <!> <!> <!> <input class="svelte-13nvtxg"/> <textarea class="svelte-13nvtxg"></textarea> <select class="svelte-13nvtxg"><option class="svelte-13nvtxg">Zero</option><option class="svelte-13nvtxg">One</option></select> <input type="checkbox" class="svelte-13nvtxg"/> <input type="radio" class="svelte-13nvtxg"/> <div contenteditable="" class="svelte-13nvtxg">editable</div> <video class="svelte-13nvtxg"></video> <div class="svelte-13nvtxg">action target</div> <div class="svelte-13nvtxg">transition target</div> <div class="svelte-13nvtxg">in/out target</div> <!> <!> <!> <!> <!> <!> <button class="svelte-13nvtxg">Update store</button> <p class="svelte-13nvtxg"> </p> <!></div>`, 2), App[$.FILENAME], [[
	187,
	0,
	[
		[189, 4],
		[190, 4],
		[195, 4],
		[261, 4],
		[262, 4],
		[
			263,
			4,
			[[264, 8], [265, 8]]
		],
		[267, 4],
		[268, 4],
		[269, 4],
		[270, 4],
		[272, 4],
		[273, 4],
		[274, 4],
		[289, 4],
		[290, 4]
	]
]]);
const $$css = {
	hash: "svelte-13nvtxg",
	code: "body {margin:0; font-family:\"IBM Plex Sans\", sans-serif; background:#f5f1e8;}.benchmark-host {color:#3f2a18;}.benchmark-reset {box-sizing:border-box;}@keyframes svelte-13nvtxg-pulse {0% {opacity:0.4; transform:scale(0.98);}100% {opacity:1; transform:scale(1);}}@keyframes marquee {from {transform:translateX(0);}to {transform:translateX(12px);}}.chunk-shell.svelte-13nvtxg {padding:16px; margin:12px 0; border:1px solid #d9c7ab; background:linear-gradient(180deg, #fffdf9 0%, #f4ead9 100%);}.chunk-shell.svelte-13nvtxg :is(.badge:where(.svelte-13nvtxg), .card:where(.svelte-13nvtxg), .summary:where(.svelte-13nvtxg)) {border-radius:10px;}.summary.svelte-13nvtxg span:where(.svelte-13nvtxg) {display:inline-block; margin-right:8px;}.item-less.svelte-13nvtxg {color:#7a4f2a;}[data-index].svelte-13nvtxg {color:var(--custom, #5c4634);}"
};
export default function App($$anchor, $$props) {
	const propsId = $.props_id();
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	const $metrics = () => ($.validate_store(metrics, "metrics"), $.store_get(metrics, "$metrics", $$stores));
	const $labelStore = () => ($.validate_store(labelStore, "labelStore"), $.store_get(labelStore, "$labelStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const binding_group = [];
	const metricSummary = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let label = () => $$arg0?.().label;
		label();
		let values = $.derived_safe_equal(() => $.fallback($$arg0?.().values, () => [$.get(counter)], true));
		$.get(values);
		let id = $.derived_safe_equal(() => $.fallback($.fallback($$arg0?.().meta, () => ({}), true).id, propsId));
		$.get(id);
		var section = root_4();
		var h4 = $.child(section);
		var text_4 = $.child(h4, true);
		$.reset(h4);
		var node_2 = $.sibling(h4, 2);
		$.add_svelte_meta(() => $.each(node_2, 17, () => $.get(values), $.index, ($$anchor, value, index) => {
			var span_1 = root_5();
			var text_5 = $.child(span_1);
			$.reset(span_1);
			$.template_effect(() => $.set_text(text_5, `${index}: ${$.get(value) ?? ""}`));
			$.append($$anchor, span_1);
		}), "each", App, 181, 8);
		$.reset(section);
		$.template_effect(() => {
			$.set_attribute(section, "data-id", $.get(id));
			$.set_text(text_4, label());
		});
		$.append($$anchor, section);
	});
	let title = $.prop($$props, "title", 7, "Default Title"), count = $.prop($$props, "count", 7, 0), items = $.prop($$props, "items", 23, () => []), config = $.prop($$props, "config", 31, () => $.tag_proxy($.proxy({}), "config")), multiplier = $.prop($$props, "multiplier", 7, 2), visible = $.prop($$props, "visible", 15, false), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"$$host",
		"title",
		"count",
		"items",
		"config",
		"multiplier",
		"visible"
	], "rest");
	let state = $.tag($.state(""), "state");
	let counter = $.tag($.state(0), "counter");
	let rawData = {
		x: 1,
		y: 2
	};
	let checked = $.tag($.state(false), "checked");
	let group = $.tag($.state($.proxy([])), "group");
	let volume = $.tag($.state(.5), "volume");
	let selected = $.tag($.state("opt-0"), "selected");
	let inputEl;
	let componentRef;
	let dynamicEl;
	let metrics = writable([
		1,
		2,
		3
	]);
	let labelStore = writable("ready");
	/** @type {Function | undefined} */
	let show;
	$.set(counter, 10);
	let doubled = $.tag($.derived(() => count() * multiplier()), "doubled");
	let computed = $.tag($.derived(() => {
		return items().length * multiplier() + $.get(counter);
	}), "computed");
	let moduleSummary = $.tag($.derived(() => moduleLabel(title()) + ":" + MODULE_SCALE), "moduleSummary");
	let storeSummary = $.tag($.derived(() => $metrics().length + ":" + $labelStore()), "storeSummary");
	let snapshot = $.snapshot(rawData);
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", "Title:", title(), "Count:", count()));
	});
	$.user_pre_effect(() => {
		console.log(...$.log_if_contains_state("log", "Pre effect:", $.get(counter)));
	});
	let tracking = $.effect_tracking();
	$.inspect(() => [$.get(counter), $.get(doubled)], (...$$args) => console.log(...$$args), true);
	const APP_VERSION = "1.0.0";
	function formatTitle(prefix) {
		return prefix + ": " + title();
	}
	function addMetric() {
		$.store_set(metrics, [...$metrics(), $.get(counter)]);
		$.store_set(labelStore, title());
	}
	function action(node, arg) {
		return { destroy() {} };
	}
	function handleClick(e) {
		$.update(counter);
	}
	function getHandler() {
		return handleClick;
	}
	function handleError(error) {
		console.error(...$.log_if_contains_state("error", error));
	}
	let promise = Promise.resolve(42);
	var $$exports = {
		get APP_VERSION() {
			return APP_VERSION;
		},
		get formatTitle() {
			return formatTitle;
		},
		get title() {
			return title();
		},
		set title($$value = "Default Title") {
			title($$value);
			$.flush();
		},
		get count() {
			return count();
		},
		set count($$value = 0) {
			count($$value);
			$.flush();
		},
		get items() {
			return items();
		},
		set items($$value = []) {
			items($$value);
			$.flush();
		},
		get config() {
			return config();
		},
		set config($$value = {}) {
			config($$value);
			$.flush();
		},
		get multiplier() {
			return multiplier();
		},
		set multiplier($$value = 2) {
			multiplier($$value);
			$.flush();
		},
		get visible() {
			return visible();
		},
		set visible($$value = false) {
			visible($$value);
			$.flush();
		},
		...$.legacy_api()
	};
	var div_1 = root();
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = `${title() ?? ""} - Benchmark`;
		});
		$.append($$anchor, fragment);
	});
	$.event("scroll", $.window, handleClick);
	$.event("visibilitychange", $.document, handleClick);
	$.event("mouseenter", $.document.body, handleClick);
	$.action($.document.body, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => $.get(state));
	$.template_effect(() => {
		console.log({
			counter: $.snapshot($.get(counter)),
			state: $.snapshot($.get(state))
		});
		debugger;
	});
	var text_6 = $.child(div_1);
	var p_1 = $.sibling(text_6);
	var text_7 = $.child(p_1);
	$.reset(p_1);
	var p_2 = $.sibling(p_1, 2);
	var text_8 = $.child(p_2);
	$.reset(p_2);
	var node_3 = $.sibling(p_2, 2);
	$.html(node_3, () => "<b>raw html chunk 0</b>");
	var div_2 = $.sibling(node_3, 2);
	let classes_1;
	var event_handler = $.derived(getHandler);
	let styles;
	var node_4 = $.sibling($.child(div_2));
	{
		var consequent = ($$anchor) => {
			const localLen = $.tag($.derived(() => $.get(state).length), "localLen");
			$.get(localLen);
			var span_2 = root_6();
			var text_9 = $.child(span_2);
			$.reset(span_2);
			$.template_effect(() => {
				$.set_attribute(span_2, "title", `${title() ?? ""}: ${$.get(doubled) ?? ""}`);
				$.set_attribute(span_2, "state", $.get(state));
				$.set_attribute(span_2, "counter", $.get(counter));
				$.set_attribute(span_2, "count", count());
				$.set_text(text_9, `Duis aute irure dolor: ${$.get(localLen) ?? ""}. Chunk 0.`);
			});
			$.append($$anchor, span_2);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = root_7();
			var div_3 = $.first_child(fragment_1);
			var input = $.child(div_3);
			$.remove_input_defaults(input);
			$.reset(div_3);
			var node_5 = $.sibling(div_3, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_8();
					$.template_effect(() => $.set_attribute(h1, "state", $.get(state)));
					$.append($$anchor, h1);
				};
				var consequent_2 = ($$anchor) => {
					var text_10 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_10);
				};
				var alternate = ($$anchor) => {
					var h2 = root_10();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_5, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_1);
					else if ($.get(counter) == 100) $$render(consequent_2, 1);
					else $$render(alternate, -1);
				}), "if", App, 224, 12);
			}
			$.template_effect(() => {
				$.set_attribute(input, "title", title());
				$.set_attribute(input, "state", $.get(state));
				$.set_value(input, count());
			});
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_4, ($$render) => {
			if ($.get(state)) $$render(consequent);
			else $$render(alternate_1, -1);
		}), "if", App, 214, 8);
	}
	$.reset(div_2);
	$.bind_this(div_2, ($$value) => dynamicEl = $$value, () => dynamicEl);
	var node_6 = $.sibling(div_2, 2);
	$.add_svelte_meta(() => $.key(node_6, () => $.get(counter), ($$anchor) => {
		var p_3 = root_11();
		var text_11 = $.child(p_3);
		$.reset(p_3);
		$.template_effect(() => $.set_text(text_11, `Keyed content chunk 0: ${$.get(counter) ?? ""}`));
		$.transition(3, p_3, () => slide);
		$.append($$anchor, p_3);
	}), "key", App, 236, 4);
	var node_7 = $.sibling(node_6, 2);
	$.add_svelte_meta(() => $.each(node_7, 27, items, (item) => item.id, ($$anchor, item, idx) => {
		const itemLabel = $.tag($.derived(() => `${$.get(idx)}:${$.get(item).name}`), "itemLabel");
		$.get(itemLabel);
		var p_4 = root_12();
		$.attribute_effect(p_4, () => ({
			...rest,
			"data-index": `chunk-0-${$.get(idx) ?? ""}`
		}), void 0, void 0, void 0, "svelte-13nvtxg");
		var text_12 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_12, $.get(itemLabel)));
		$.animation(p_4, () => flip, null);
		$.append($$anchor, p_4);
	}), "each", App, 240, 4);
	var node_8 = $.sibling(node_7, 2);
	$.add_svelte_meta(() => $.each(node_8, 17, items, $.index, ($$anchor, $$item) => {
		var span_3 = root_13();
		$.append($$anchor, span_3);
	}), "each", App, 245, 4);
	var node_9 = $.sibling(node_8, 2);
	$.add_svelte_meta(() => $.await(node_9, () => promise, ($$anchor) => {
		var p_7 = root_16();
		$.append($$anchor, p_7);
	}, ($$anchor, value) => {
		var p_5 = root_14();
		var text_13 = $.child(p_5);
		$.reset(p_5);
		$.template_effect(() => $.set_text(text_13, `Resolved: ${$.get(value) ?? ""}`));
		$.append($$anchor, p_5);
	}, ($$anchor, error) => {
		var p_6 = root_15();
		var text_14 = $.child(p_6);
		$.reset(p_6);
		$.template_effect(() => $.set_text(text_14, `Error: ${$.get(error).message ?? ""}`));
		$.append($$anchor, p_6);
	}), "await", App, 249, 4);
	var node_10 = $.sibling(node_9, 2);
	$.add_svelte_meta(() => $.await(node_10, () => promise, null, ($$anchor, quickValue) => {
		var p_8 = root_17();
		var text_15 = $.child(p_8);
		$.reset(p_8);
		$.template_effect(() => $.set_text(text_15, `Quick resolved: ${$.get(quickValue) ?? ""}`));
		$.append($$anchor, p_8);
	}), "await", App, 257, 4);
	var input_1 = $.sibling(node_10, 2);
	$.remove_input_defaults(input_1);
	var textarea = $.sibling(input_1, 2);
	$.remove_textarea_child(textarea);
	var select = $.sibling(textarea, 2);
	var option = $.child(select);
	option.value = option.__value = "opt-0";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "opt-1";
	$.reset(select);
	var input_2 = $.sibling(select, 2);
	$.remove_input_defaults(input_2);
	var input_3 = $.sibling(input_2, 2);
	$.remove_input_defaults(input_3);
	input_3.value = input_3.__value = "opt-0";
	var div_4 = $.sibling(input_3, 2);
	$.bind_this(div_4, ($$value) => inputEl = $$value, () => inputEl);
	var video = $.sibling(div_4, 2);
	var div_5 = $.sibling(video, 2);
	$.action(div_5, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => $.get(state));
	var div_6 = $.sibling(div_5, 2);
	var div_7 = $.sibling(div_6, 2);
	var node_11 = $.sibling(div_7, 2);
	{
		$.validate_dynamic_element_tag(() => $.get(state) ? "div" : "span");
		$.validate_void_dynamic_element(() => $.get(state) ? "div" : "span");
		$.element(node_11, () => $.get(state) ? "div" : "span", false, ($$element, $$anchor) => {
			$.set_class($$element, 0, "dynamic-0 svelte-13nvtxg");
			var text_16 = $.text();
			$.template_effect(() => $.set_text(text_16, `Dynamic element chunk 0: ${title() ?? ""}`));
			$.append($$anchor, text_16);
		}, void 0, [275, 4]);
	}
	var node_12 = $.sibling(node_11, 2);
	{
		let $0 = $.derived(getHandler);
		$.add_svelte_meta(() => $.bind_this(ChildComponent(node_12, {
			get title() {
				return title();
			},
			get onclick() {
				return $.get($0);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var strong = root_19();
				var text_17 = $.child(strong);
				$.reset(strong);
				$.template_effect(() => $.set_text(text_17, `Inline child chunk 0: ${title() ?? ""}`));
				$.append($$anchor, strong);
			}),
			$$slots: {
				default: true,
				footer: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
					var div_8 = root_20();
					var text_18 = $.child(div_8);
					$.reset(div_8);
					$.template_effect(() => $.set_text(text_18, `Footer chunk 0: ${$.get(counter) ?? ""}`));
					$.append($$anchor, div_8);
				})
			}
		}), ($$value) => componentRef = $$value, () => componentRef), "component", App, 279, 4, { componentTag: "ChildComponent" });
	}
	var node_13 = $.sibling(node_12, 2);
	$.add_svelte_meta(() => badge(node_13, () => "chunk-0", () => "secondary"), "render", App, 284, 4);
	var node_14 = $.sibling(node_13, 2);
	$.add_svelte_meta(() => card(node_14, title, () => "Content for chunk 0"), "render", App, 285, 4);
	var node_15 = $.sibling(node_14, 2);
	$.add_svelte_meta(() => metricSummary(node_15, () => ({
		label: title(),
		values: [
			count(),
			$.get(doubled),
			$.get(counter)
		],
		meta: { id: propsId }
	})), "render", App, 286, 4);
	var node_16 = $.sibling(node_15, 2);
	$.add_svelte_meta(() => show?.(node_16), "render", App, 287, 4);
	var button = $.sibling(node_16, 2);
	var p_9 = $.sibling(button, 2);
	var text_19 = $.child(p_9);
	$.reset(p_9);
	var node_17 = $.sibling(p_9, 2);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var p_10 = root_21();
			var text_20 = $.child(p_10);
			$.reset(p_10);
			$.template_effect(() => $.set_text(text_20, `Error in chunk 0: ${error().message ?? ""}`));
			$.append($$anchor, p_10);
		});
		$.boundary(node_17, {
			onerror: handleError,
			failed
		}, ($$anchor) => {
			var p_11 = root_22();
			var text_21 = $.child(p_11);
			$.reset(p_11);
			$.template_effect(() => $.set_text(text_21, `Boundary chunk 0: ${title() ?? ""}`));
			$.append($$anchor, p_11);
		});
	}
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text_6, `Chunk 0: Lorem ${$.get(state) ?? ""} + ${$.get(state) ?? ""} = Ipsum; `);
		$.set_text(text_7, `Props: title=${title() ?? ""}, count=${count() ?? ""}, doubled=${$.get(doubled) ?? ""}, computed=${$.get(computed) ?? ""}`);
		$.set_text(text_8, `Module: ${$.get(moduleSummary) ?? ""} | Store: ${$.get(storeSummary) ?? ""} | Label: ${$labelStore() ?? ""}`);
		classes_1 = $.set_class(div_2, 1, $.clsx({
			active: $.get(checked),
			big: $.get(counter) > 10
		}), "svelte-13nvtxg", classes_1, {
			state: $.get(state),
			staticly: true,
			invinsible,
			reactive: $.get(counter)
		});
		styles = $.set_style(div_2, "", styles, {
			color: $.get(state),
			"font-size": "14px",
			opacity: $.get(counter) / 100,
			"--custom": "value-0"
		});
		$.set_text(text_19, `Metric count: ${$metrics().length ?? ""}`);
	});
	$.delegated("click", div_2, handleClick);
	$.event("scroll", div_2, handleClick);
	$.event("click", div_2, handleClick, true);
	$.event("focus", div_2, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [208, 17], true, true);
	});
	$.bind_value(input_1, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_value(textarea, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_select_value(select, function get() {
		return $.get(selected);
	}, function set($$value) {
		$.set(selected, $$value);
	});
	$.bind_checked(input_2, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.bind_group(binding_group, [], input_3, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_element_size(div_4, "clientWidth", function set($$value) {
		$.set(counter, $$value);
	});
	$.bind_content_editable("innerHTML", div_4, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_volume(video, function get() {
		return $.get(volume);
	}, function set($$value) {
		$.set(volume, $$value);
	});
	$.bind_paused(video, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.transition(3, div_6, () => fade);
	$.transition(1, div_7, () => fly, () => ({ y: 200 }));
	$.transition(2, div_7, () => fade);
	$.delegated("click", button, addMetric);
	$.append($$anchor, div_1);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
$.create_custom_element(App, {
	title: {},
	count: {},
	items: {},
	config: {},
	multiplier: {},
	visible: {}
}, [], ["APP_VERSION", "formatTitle"], { mode: "open" });
