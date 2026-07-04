import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
const badge = $.wrap_snippet(App, function($$anchor, text = $.noop, variant = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root_1();
	let classes;
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => {
		classes = $.set_class(span, 1, "badge svelte-13nvtxg", null, classes, {
			primary: $.strict_equals(variant(), "primary"),
			secondary: $.strict_equals(variant(), "secondary")
		});
		$.set_text(text_1, text());
	});
	$.append($$anchor, span);
});
const card = $.wrap_snippet(App, function($$anchor, heading = $.noop, body = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root_2();
	var h3 = $.child(div);
	var text_2 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_3 = $.child(p, true);
	$.reset(p);
	var node_1 = $.sibling(p, 2);
	$.add_svelte_meta(() => badge(node_1, () => "new", () => "primary"), "render", App, 173, 8);
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
var root = $.add_locations($.from_html(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/> <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`, 1), App[$.FILENAME], [[155, 4], [156, 4]]);
var root_1 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[164, 4]]);
var root_2 = $.add_locations($.from_html(`<div class="card svelte-13nvtxg"><h3 class="svelte-13nvtxg"> </h3> <p class="svelte-13nvtxg"> </p> <!></div>`), App[$.FILENAME], [[
	170,
	4,
	[[171, 8], [172, 8]]
]]);
var root_3 = $.add_locations($.from_html(`<span class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[181, 12]]);
var root_4 = $.add_locations($.from_html(`<section class="summary svelte-13nvtxg"><h4 class="svelte-13nvtxg"> </h4> <!></section>`), App[$.FILENAME], [[
	178,
	4,
	[[179, 8]]
]]);
var root_5 = $.add_locations($.from_html(`<span empty="" class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[215, 12]]);
var root_6 = $.add_locations($.from_html(`<h1 class="svelte-13nvtxg">Lorem ipsum dolor sit amet. Chunk 0.</h1>`), App[$.FILENAME], [[224, 16]]);
var root_7 = $.add_locations($.from_html(`<h2 class="svelte-13nvtxg">EMPTY</h2>`), App[$.FILENAME], [[230, 16]]);
var root_8 = $.add_locations($.from_html(`<div class="svelte-13nvtxg"><input class="svelte-13nvtxg"/></div> <!>`, 1), App[$.FILENAME], [[
	219,
	12,
	[[220, 16]]
]]);
var root_9 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[236, 8]]);
var root_10 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[241, 8]]);
var root_11 = $.add_locations($.from_html(`<span class="item-less svelte-13nvtxg">Repeated shell chunk 0</span>`), App[$.FILENAME], [[245, 8]]);
var root_12 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[251, 8]]);
var root_13 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[253, 8]]);
var root_14 = $.add_locations($.from_html(`<p class="svelte-13nvtxg">Loading chunk 0...</p>`), App[$.FILENAME], [[249, 8]]);
var root_15 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[257, 8]]);
var root_16 = $.add_locations($.from_html(`<strong class="svelte-13nvtxg"> </strong>`), App[$.FILENAME], [[279, 8]]);
var root_17 = $.add_locations($.from_html(`<div slot="footer" class="svelte-13nvtxg"> </div>`), App[$.FILENAME], [[280, 8]]);
var root_18 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[294, 12]]);
var root_19 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[292, 8]]);
var root_20 = $.add_locations($.from_html(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0"> <p class="svelte-13nvtxg"> </p> <p class="svelte-13nvtxg"> </p> <!> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. <!></div> <!> <!> <!> <!> <!> <input class="svelte-13nvtxg"/> <textarea class="svelte-13nvtxg"></textarea> <select class="svelte-13nvtxg"><option class="svelte-13nvtxg">Zero</option><option class="svelte-13nvtxg">One</option></select> <input type="checkbox" class="svelte-13nvtxg"/> <input type="radio" class="svelte-13nvtxg"/> <div contenteditable="" class="svelte-13nvtxg">editable</div> <video class="svelte-13nvtxg"></video> <div class="svelte-13nvtxg">action target</div> <div class="svelte-13nvtxg">transition target</div> <div class="svelte-13nvtxg">in/out target</div> <!> <!> <!> <!> <!> <!> <button class="svelte-13nvtxg">Update store</button> <p class="svelte-13nvtxg"> </p> <!></div>`, 2), App[$.FILENAME], [[
	186,
	0,
	[
		[188, 4],
		[189, 4],
		[194, 4],
		[260, 4],
		[261, 4],
		[
			262,
			4,
			[[263, 8], [264, 8]]
		],
		[266, 4],
		[267, 4],
		[268, 4],
		[269, 4],
		[271, 4],
		[272, 4],
		[273, 4],
		[288, 4],
		[289, 4]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $bindable = () => ($.validate_store(bindable, "bindable"), $.store_get(bindable, "$bindable", $$stores));
	const $props = () => ($.validate_store(props, "props"), $.store_get(props, "$props", $$stores));
	const $state = () => ($.validate_store($.get(state), "state"), $.store_get($.get(state), "$state", $$stores));
	const $derived = () => ($.validate_store(derived, "derived"), $.store_get(derived, "$derived", $$stores));
	const $metrics = () => ($.validate_store(metrics, "metrics"), $.store_get(metrics, "$metrics", $$stores));
	const $labelStore = () => ($.validate_store(labelStore, "labelStore"), $.store_get(labelStore, "$labelStore", $$stores));
	const $effect = () => ($.validate_store(effect, "effect"), $.store_get(effect, "$effect", $$stores));
	const $inspect = () => ($.validate_store(inspect, "inspect"), $.store_get(inspect, "$inspect", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const binding_group = [];
	const metricSummary = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let label = () => ($$arg0?.()).label;
		label();
		let values = $.derived_safe_equal(() => $.fallback(($$arg0?.()).values, () => [$.get(counter)], true));
		$.get(values);
		let id = $.derived_safe_equal(() => $.fallback($.fallback(($$arg0?.()).meta, () => ({}), true).id, propsId));
		$.get(id);
		var section = root_4();
		var h4 = $.child(section);
		var text_4 = $.child(h4, true);
		$.reset(h4);
		var node_2 = $.sibling(h4, 2);
		$.add_svelte_meta(() => $.each(node_2, 1, () => $.get(values), $.index, ($$anchor, value, index) => {
			var span_1 = root_3();
			var text_5 = $.child(span_1);
			$.reset(span_1);
			$.template_effect(() => $.set_text(text_5, `${index}: ${$.get(value) ?? ""}`));
			$.append($$anchor, span_1);
		}), "each", App, 180, 8);
		$.reset(section);
		$.template_effect(() => {
			$.set_attribute(section, "data-id", $.get(id));
			$.set_text(text_4, label());
		});
		$.append($$anchor, section);
	});
	let { title = "Default Title", count = 0, items = [], config = $bindable()({}), multiplier = 2, visible = $bindable()(false), ...rest } = $props()();
	const propsId = $props().id();
	let state = $.tag($.mutable_source($state()("")), "state");
	let counter = $.tag($.mutable_source($state()(0)), "counter");
	let rawData = $state().raw({
		x: 1,
		y: 2
	});
	let checked = $.tag($.mutable_source($state()(false)), "checked");
	let group = $.tag($.mutable_source($state()([])), "group");
	let volume = $.tag($.mutable_source($state()(.5)), "volume");
	let selected = $.tag($.mutable_source($state()("opt-0")), "selected");
	let inputEl = $.tag($.mutable_source(), "inputEl");
	let componentRef = $.tag($.mutable_source(), "componentRef");
	let dynamicEl = $.tag($.mutable_source(), "dynamicEl");
	let metrics = writable([
		1,
		2,
		3
	]);
	let labelStore = writable("ready");
	let show;
	$.set(counter, 10);
	let doubled = $derived()(count * multiplier);
	let computed = $derived().by(() => {
		return items.length * multiplier + $.get(counter);
	});
	let moduleSummary = $derived()(moduleLabel(title) + ":" + MODULE_SCALE);
	let storeSummary = $derived()($metrics().length + ":" + $labelStore());
	let snapshot = $state().snapshot(rawData);
	$effect()(() => {
		console.log(...$.log_if_contains_state("log", "Title:", title, "Count:", count));
	});
	$effect().pre(() => {
		console.log(...$.log_if_contains_state("log", "Pre effect:", $.get(counter)));
	});
	let tracking = $effect().tracking();
	$inspect()($.get(counter), doubled);
	const APP_VERSION = "1.0.0";
	function formatTitle(prefix) {
		return prefix + ": " + title;
	}
	function addMetric() {
		$.store_set(metrics, [...$metrics(), $.get(counter)]);
		$.store_set(labelStore, title);
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
		...$.legacy_api(),
		get APP_VERSION() {
			return APP_VERSION;
		},
		get formatTitle() {
			return formatTitle;
		}
	};
	$.init();
	var div_1 = root_20();
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = `${title ?? ""} - Benchmark`;
		});
		$.append($$anchor, fragment);
	});
	$.event("scroll", $.window, handleClick);
	$.event("visibilitychange", $.document, handleClick);
	$.event("mouseenter", $.document.body, handleClick);
	$.action($.document.body, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => $.get(state));
	$.template_effect(() => {
		console.log({
			counter: $.untrack(() => $.snapshot($.get(counter))),
			state: $.untrack(() => $.snapshot($.get(state)))
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
			const localLen = $.tag($.derived_safe_equal(() => ($.get(state), $.untrack(() => $.get(state).length))), "localLen");
			$.get(localLen);
			var span_2 = root_5();
			var text_9 = $.child(span_2);
			$.reset(span_2);
			$.template_effect(() => {
				$.set_attribute(span_2, "title", `${title ?? ""}: ${doubled ?? ""}`);
				$.set_attribute(span_2, "state", $.get(state));
				$.set_attribute(span_2, "counter", $.get(counter));
				$.set_attribute(span_2, "count", count);
				$.set_text(text_9, `Duis aute irure dolor: ${$.get(localLen) ?? ""}. Chunk 0.`);
			});
			$.append($$anchor, span_2);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = root_8();
			var div_3 = $.first_child(fragment_1);
			var input = $.child(div_3);
			$.remove_input_defaults(input);
			$.reset(div_3);
			var node_5 = $.sibling(div_3, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_6();
					$.template_effect(() => $.set_attribute(h1, "state", $.get(state)));
					$.append($$anchor, h1);
				};
				var consequent_2 = ($$anchor) => {
					var text_10 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_10);
				};
				var alternate = ($$anchor) => {
					var h2 = root_7();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_5, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_1);
					else if ($.equals($.get(counter), 100)) $$render(consequent_2, 1);
					else $$render(alternate, -1);
				}), "if", App, 223, 12);
			}
			$.template_effect(() => {
				$.set_attribute(input, "title", title);
				$.set_attribute(input, "state", $.get(state));
				$.set_value(input, count);
			});
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_4, ($$render) => {
			if ($.get(state)) $$render(consequent);
			else $$render(alternate_1, -1);
		}), "if", App, 213, 8);
	}
	$.reset(div_2);
	$.bind_this(div_2, ($$value) => $.set(dynamicEl, $$value), () => $.get(dynamicEl));
	var node_6 = $.sibling(div_2, 2);
	$.add_svelte_meta(() => $.key(node_6, () => $.get(counter), ($$anchor) => {
		var p_3 = root_9();
		var text_11 = $.child(p_3);
		$.reset(p_3);
		$.template_effect(() => $.set_text(text_11, `Keyed content chunk 0: ${$.get(counter) ?? ""}`));
		$.transition(3, p_3, () => slide);
		$.append($$anchor, p_3);
	}), "key", App, 235, 4);
	var node_7 = $.sibling(node_6, 2);
	$.add_svelte_meta(() => $.each(node_7, 11, () => items, (item) => item.id, ($$anchor, item, idx) => {
		const itemLabel = $.tag($.derived_safe_equal(() => ($.deep_read_state($.get(idx)), $.get(item), $.untrack(() => `${$.get(idx)}:${$.get(item).name}`))), "itemLabel");
		$.get(itemLabel);
		var p_4 = root_10();
		$.attribute_effect(p_4, () => ({
			...rest,
			"data-index": `chunk-0-${$.get(idx) ?? ""}`
		}), void 0, void 0, void 0, "svelte-13nvtxg");
		var text_12 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_12, $.get(itemLabel)));
		$.animation(p_4, () => flip, null);
		$.append($$anchor, p_4);
	}), "each", App, 239, 4);
	var node_8 = $.sibling(node_7, 2);
	$.add_svelte_meta(() => $.each(node_8, 1, () => items, $.index, ($$anchor, $$item) => {
		var span_3 = root_11();
		$.append($$anchor, span_3);
	}), "each", App, 244, 4);
	var node_9 = $.sibling(node_8, 2);
	$.add_svelte_meta(() => $.await(node_9, () => promise, ($$anchor) => {
		var p_7 = root_14();
		$.append($$anchor, p_7);
	}, ($$anchor, value) => {
		var p_5 = root_12();
		var text_13 = $.child(p_5);
		$.reset(p_5);
		$.template_effect(() => $.set_text(text_13, `Resolved: ${$.get(value) ?? ""}`));
		$.append($$anchor, p_5);
	}, ($$anchor, error) => {
		var p_6 = root_13();
		var text_14 = $.child(p_6);
		$.reset(p_6);
		$.template_effect(() => $.set_text(text_14, `Error: ${($.deep_read_state($.get(error)), $.untrack(() => $.get(error).message)) ?? ""}`));
		$.append($$anchor, p_6);
	}), "await", App, 248, 4);
	var node_10 = $.sibling(node_9, 2);
	$.add_svelte_meta(() => $.await(node_10, () => promise, null, ($$anchor, quickValue) => {
		var p_8 = root_15();
		var text_15 = $.child(p_8);
		$.reset(p_8);
		$.template_effect(() => $.set_text(text_15, `Quick resolved: ${$.get(quickValue) ?? ""}`));
		$.append($$anchor, p_8);
	}), "await", App, 256, 4);
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
	$.bind_this(div_4, ($$value) => $.set(inputEl, $$value), () => $.get(inputEl));
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
			$.template_effect(() => $.set_text(text_16, `Dynamic element chunk 0: ${title ?? ""}`));
			$.append($$anchor, text_16);
		}, void 0, [274, 4]);
	}
	var node_12 = $.sibling(node_11, 2);
	{
		let $0 = $.derived_safe_equal(() => $.untrack(getHandler));
		$.add_svelte_meta(() => $.bind_this(ChildComponent(node_12, {
			get title() {
				return title;
			},
			get onclick() {
				return $.get($0);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var strong = root_16();
				var text_17 = $.child(strong);
				$.reset(strong);
				$.template_effect(() => $.set_text(text_17, `Inline child chunk 0: ${title ?? ""}`));
				$.append($$anchor, strong);
			}),
			$$slots: {
				default: true,
				footer: ($$anchor, $$slotProps) => {
					var div_8 = root_17();
					var text_18 = $.child(div_8);
					$.reset(div_8);
					$.template_effect(() => $.set_text(text_18, `Footer chunk 0: ${$.get(counter) ?? ""}`));
					$.append($$anchor, div_8);
				}
			},
			$$legacy: true
		}), ($$value) => $.set(componentRef, $$value), () => $.get(componentRef)), "component", App, 278, 4, { componentTag: "ChildComponent" });
	}
	var node_13 = $.sibling(node_12, 2);
	$.add_svelte_meta(() => badge(node_13, () => "chunk-0", () => "secondary"), "render", App, 283, 4);
	var node_14 = $.sibling(node_13, 2);
	$.add_svelte_meta(() => card(node_14, () => title, () => "Content for chunk 0"), "render", App, 284, 4);
	var node_15 = $.sibling(node_14, 2);
	$.add_svelte_meta(() => metricSummary(node_15, () => ({
		label: title,
		values: [
			count,
			doubled,
			$.get(counter)
		],
		meta: { id: propsId }
	})), "render", App, 285, 4);
	var node_16 = $.sibling(node_15, 2);
	$.add_svelte_meta(() => show?.(node_16), "render", App, 286, 4);
	var button = $.sibling(node_16, 2);
	var p_9 = $.sibling(button, 2);
	var text_19 = $.child(p_9);
	$.reset(p_9);
	var node_17 = $.sibling(p_9, 2);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var p_10 = root_18();
			var text_20 = $.child(p_10);
			$.reset(p_10);
			$.template_effect(() => $.set_text(text_20, `Error in chunk 0: ${(error(), $.untrack(() => error().message)) ?? ""}`));
			$.append($$anchor, p_10);
		});
		$.boundary(node_17, {
			onerror: handleError,
			failed
		}, ($$anchor) => {
			var p_11 = root_19();
			var text_21 = $.child(p_11);
			$.reset(p_11);
			$.template_effect(() => $.set_text(text_21, `Boundary chunk 0: ${title ?? ""}`));
			$.append($$anchor, p_11);
		});
	}
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text_6, `Chunk 0: Lorem ${$.get(state) ?? ""} + ${$.get(state) ?? ""} = Ipsum; `);
		$.set_text(text_7, `Props: title=${title ?? ""}, count=${count ?? ""}, doubled=${doubled ?? ""}, computed=${computed ?? ""}`);
		$.set_text(text_8, `Module: ${moduleSummary ?? ""} | Store: ${storeSummary ?? ""} | Label: ${$labelStore() ?? ""}`);
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
		$.set_text(text_19, `Metric count: ${($metrics(), $.untrack(() => $metrics().length)) ?? ""}`);
	});
	$.delegated("click", div_2, handleClick);
	$.event("scroll", div_2, handleClick);
	$.event("click", div_2, handleClick, true);
	$.event("focus", div_2, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [207, 17], true, true);
	});
	$.bind_value(input_1, function get() {
		return $.get(state);
	}, function set($$value) {
		$.store_unsub($.set(state, $$value), "$state", $$stores);
	});
	$.bind_value(textarea, function get() {
		return $.get(state);
	}, function set($$value) {
		$.store_unsub($.set(state, $$value), "$state", $$stores);
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
		$.store_unsub($.set(state, $$value), "$state", $$stores);
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
	$.bind_prop($$props, "APP_VERSION", APP_VERSION);
	$.bind_prop($$props, "formatTitle", formatTitle);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
