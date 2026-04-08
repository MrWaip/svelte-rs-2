import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
export const BENCHMARK_KIND = "compiler";
export const MODULE_SCALE = 3;
export function moduleLabel(name) {
	return `${BENCHMARK_KIND}:${name}`;
}
const badge = ($$anchor, text = $.noop, variant = $.noop) => {
	var span_1 = root_4();
	let classes;
	var text_3 = $.child(span_1, true);
	$.reset(span_1);
	$.template_effect(() => {
		classes = $.set_class(span_1, 1, "badge", null, classes, {
			primary: variant() === "primary",
			secondary: variant() === "secondary"
		});
		$.set_text(text_3, text());
	});
	$.append($$anchor, span_1);
};
const card = ($$anchor, heading = $.noop, body = $.noop) => {
	var div = root_5();
	var h3 = $.child(div);
	var text_4 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_5 = $.child(p, true);
	$.reset(p);
	var node_2 = $.sibling(p, 2);
	badge(node_2, () => "new", () => "primary");
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text_4, heading());
		$.set_text(text_5, body());
	});
	$.append($$anchor, div);
};
var root_1 = $.from_html(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/> <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`, 1);
var root_3 = $.from_html(`<span class="svelte-13nvtxg"> </span>`);
var root_2 = $.from_html(`<section class="summary svelte-13nvtxg"><h4 class="svelte-13nvtxg"> </h4> <!></section>`);
var root_4 = $.from_html(`<span class="svelte-13nvtxg"> </span>`);
var root_5 = $.from_html(`<div class="card svelte-13nvtxg"><h3 class="svelte-13nvtxg"> </h3> <p class="svelte-13nvtxg"> </p> <!></div>`);
var root_6 = $.from_html(`<span empty="" class="svelte-13nvtxg"> </span>`);
var root_8 = $.from_html(`<h1 class="svelte-13nvtxg">Lorem ipsum dolor sit amet. Chunk 0.</h1>`);
var root_10 = $.from_html(`<h2 class="svelte-13nvtxg">EMPTY</h2>`);
var root_7 = $.from_html(`<div class="svelte-13nvtxg"><input class="svelte-13nvtxg"/></div> <!>`, 1);
var root_11 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root_12 = $.from_html(`<p> </p>`);
var root_13 = $.from_html(`<span class="item-less svelte-13nvtxg">Repeated shell chunk 0</span>`);
var root_14 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root_15 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root_16 = $.from_html(`<p class="svelte-13nvtxg">Loading chunk 0...</p>`);
var root_17 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root_19 = $.from_html(`<strong class="svelte-13nvtxg"> </strong>`);
var root_20 = $.from_html(`<div slot="footer" class="svelte-13nvtxg"> </div>`);
var root_21 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root_22 = $.from_html(`<p class="svelte-13nvtxg"> </p>`);
var root = $.from_html(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0"> <p class="svelte-13nvtxg"> </p> <p class="svelte-13nvtxg"> </p> <!> <div class="svelte-13nvtxg">Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. <!></div> <!> <!> <!> <!> <!> <input class="svelte-13nvtxg"/> <textarea class="svelte-13nvtxg"></textarea> <select class="svelte-13nvtxg"><option class="svelte-13nvtxg">Zero</option><option class="svelte-13nvtxg">One</option></select> <input type="checkbox" class="svelte-13nvtxg"/> <input type="radio" class="svelte-13nvtxg"/> <div contenteditable="" class="svelte-13nvtxg">editable</div> <video class="svelte-13nvtxg"></video> <div class="svelte-13nvtxg">action target</div> <div class="svelte-13nvtxg">transition target</div> <div class="svelte-13nvtxg">in/out target</div> <!> <!> <!> <!> <!> <!> <button class="svelte-13nvtxg">Update store</button> <p class="svelte-13nvtxg"> </p> <!></div>`, 2);
export default function App($$anchor, $$props) {
	const propsId = $.props_id();
	$.push($$props, true);
	const binding_group = [];
	const $labelStore = () => $.store_get(labelStore, "$labelStore", $$stores);
	const $metrics = () => $.store_get(metrics, "$metrics", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const metricSummary = ($$anchor, $$arg0) => {
		let label = () => $$arg0?.().label;
		let values = $.derived_safe_equal(() => $.fallback($$arg0?.().values, () => [$.get(counter)], true));
		let id = $.derived_safe_equal(() => $.fallback($.fallback($$arg0?.().meta, () => ({}), true).id, propsId));
		var section = root_2();
		var h4 = $.child(section);
		var text_1 = $.child(h4, true);
		$.reset(h4);
		var node_1 = $.sibling(h4, 2);
		$.each(node_1, 17, () => $.get(values), $.index, ($$anchor, value, index) => {
			var span = root_3();
			var text_2 = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text_2, `${index}: ${$.get(value) ?? ""}`));
			$.append($$anchor, span);
		});
		$.reset(section);
		$.template_effect(() => {
			$.set_attribute(section, "data-id", $.get(id));
			$.set_text(text_1, label());
		});
		$.append($$anchor, section);
	};
	let title = $.prop($$props, "title", 3, "Default Title"), count = $.prop($$props, "count", 3, 0), items = $.prop($$props, "items", 19, () => []), config = $.prop($$props, "config", 27, () => $.proxy({})), multiplier = $.prop($$props, "multiplier", 3, 2), visible = $.prop($$props, "visible", 11, false), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"title",
		"count",
		"items",
		"config",
		"multiplier",
		"visible"
	]);
	let state = $.state("");
	let counter = $.state(0);
	let rawData = {
		x: 1,
		y: 2
	};
	let checked = $.state(false);
	let group = $.state($.proxy([]));
	let volume = $.state(.5);
	let selected = $.state("opt-0");
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
	let doubled = $.derived(() => count() * multiplier());
	let computed = $.derived(() => {
		return items().length * multiplier() + $.get(counter);
	});
	let moduleSummary = $.derived(() => moduleLabel(title()) + ":" + MODULE_SCALE);
	let storeSummary = $.derived(() => $metrics().length + ":" + $labelStore());
	let snapshot = $.snapshot(rawData);
	$.user_effect(() => {
		console.log("Title:", title(), "Count:", count());
	});
	$.user_pre_effect(() => {
		console.log("Pre effect:", $.get(counter));
	});
	let tracking = $.effect_tracking();
	;
	;
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
		console.error(error);
	}
	let promise = Promise.resolve(42);
	var $$exports = {
		APP_VERSION,
		formatTitle
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
			const localLen = $.derived(() => $.get(state).length);
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
				$.if(node_5, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_1);
					else if ($.get(counter) == 100) $$render(consequent_2, 1);
					else $$render(alternate, -1);
				});
			}
			$.template_effect(() => {
				$.set_attribute(input, "title", title());
				$.set_attribute(input, "state", $.get(state));
				$.set_value(input, count());
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node_4, ($$render) => {
			if ($.get(state)) $$render(consequent);
			else $$render(alternate_1, -1);
		});
	}
	$.reset(div_2);
	$.bind_this(div_2, ($$value) => dynamicEl = $$value, () => dynamicEl);
	var node_6 = $.sibling(div_2, 2);
	$.key(node_6, () => $.get(counter), ($$anchor) => {
		var p_3 = root_11();
		var text_11 = $.child(p_3);
		$.reset(p_3);
		$.template_effect(() => $.set_text(text_11, `Keyed content chunk 0: ${$.get(counter) ?? ""}`));
		$.transition(3, p_3, () => slide);
		$.append($$anchor, p_3);
	});
	var node_7 = $.sibling(node_6, 2);
	$.each(node_7, 27, items, (item) => item.id, ($$anchor, item, idx) => {
		const itemLabel = $.derived(() => `${$.get(idx)}:${$.get(item).name}`);
		var p_4 = root_12();
		$.attribute_effect(p_4, () => ({
			...rest,
			"data-index": `chunk-0-${$.get(idx) ?? ""}`
		}));
		var text_12 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_12, $.get(itemLabel)));
		$.animation(p_4, () => flip, null);
		$.append($$anchor, p_4);
	});
	var node_8 = $.sibling(node_7, 2);
	$.each(node_8, 17, items, $.index, ($$anchor, $$item) => {
		var span_3 = root_13();
		$.append($$anchor, span_3);
	});
	var node_9 = $.sibling(node_8, 2);
	$.await(node_9, () => promise, ($$anchor) => {
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
	});
	var node_10 = $.sibling(node_9, 2);
	$.await(node_10, () => promise, null, ($$anchor, quickValue) => {
		var p_8 = root_17();
		var text_15 = $.child(p_8);
		$.reset(p_8);
		$.template_effect(() => $.set_text(text_15, `Quick resolved: ${$.get(quickValue) ?? ""}`));
		$.append($$anchor, p_8);
	});
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
	$.element(node_11, () => $.get(state) ? "div" : "span", false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "dynamic-0");
		var text_16 = $.text();
		$.template_effect(() => $.set_text(text_16, `Dynamic element chunk 0: ${title() ?? ""}`));
		$.append($$anchor, text_16);
	});
	var node_12 = $.sibling(node_11, 2);
	{
		let $0 = $.derived(getHandler);
		$.bind_this(ChildComponent(node_12, {
			get title() {
				return title();
			},
			get onclick() {
				return $.get($0);
			},
			children: ($$anchor, $$slotProps) => {
				var strong = root_19();
				var text_17 = $.child(strong);
				$.reset(strong);
				$.template_effect(() => $.set_text(text_17, `Inline child chunk 0: ${title() ?? ""}`));
				$.append($$anchor, strong);
			},
			$$slots: {
				default: true,
				footer: ($$anchor, $$slotProps) => {
					var div_8 = root_20();
					var text_18 = $.child(div_8);
					$.reset(div_8);
					$.template_effect(() => $.set_text(text_18, `Footer chunk 0: ${$.get(counter) ?? ""}`));
					$.append($$anchor, div_8);
				}
			}
		}), ($$value) => componentRef = $$value, () => componentRef);
	}
	var node_13 = $.sibling(node_12, 2);
	badge(node_13, () => "chunk-0", () => "secondary");
	var node_14 = $.sibling(node_13, 2);
	card(node_14, title, () => "Content for chunk 0");
	var node_15 = $.sibling(node_14, 2);
	metricSummary(node_15, () => ({
		label: title(),
		values: [
			count(),
			$.get(doubled),
			$.get(counter)
		],
		meta: { id: propsId }
	}));
	var node_16 = $.sibling(node_15, 2);
	show?.(node_16);
	var button = $.sibling(node_16, 2);
	var p_9 = $.sibling(button, 2);
	var text_19 = $.child(p_9);
	$.reset(p_9);
	var node_17 = $.sibling(p_9, 2);
	{
		const failed = ($$anchor, error = $.noop) => {
			var p_10 = root_21();
			var text_20 = $.child(p_10);
			$.reset(p_10);
			$.template_effect(() => $.set_text(text_20, `Error in chunk 0: ${error().message ?? ""}`));
			$.append($$anchor, p_10);
		};
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
		}), null, classes_1, {
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
		$.get(event_handler)?.apply(this, $$args);
	});
	$.bind_value(input_1, () => $.get(state), ($$value) => $.set(state, $$value));
	$.bind_value(textarea, () => $.get(state), ($$value) => $.set(state, $$value));
	$.bind_select_value(select, () => $.get(selected), ($$value) => $.set(selected, $$value));
	$.bind_checked(input_2, () => $.get(checked), ($$value) => $.set(checked, $$value));
	$.bind_group(binding_group, [], input_3, () => $.get(group), ($$value) => $.set(group, $$value));
	$.bind_element_size(div_4, "clientWidth", ($$value) => $.set(counter, $$value));
	$.bind_content_editable("innerHTML", div_4, () => $.get(state), ($$value) => $.set(state, $$value));
	$.bind_volume(video, () => $.get(volume), ($$value) => $.set(volume, $$value));
	$.bind_paused(video, () => $.get(checked), ($$value) => $.set(checked, $$value));
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
