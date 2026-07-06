App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
$.prevent_snippet_stringification(badge);
function badge($$renderer, text, variant) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span${$.attr_class("badge svelte-13nvtxg", void 0, {
		"primary": variant === "primary",
		"secondary": variant === "secondary"
	})}>`);
	$.push_element($$renderer, "span", 164, 4);
	$$renderer.push(`${$.escape(text)}</span>`);
	$.pop_element();
}
$.prevent_snippet_stringification(card);
function card($$renderer, heading, body) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div class="card svelte-13nvtxg">`);
	$.push_element($$renderer, "div", 170, 4);
	$$renderer.push(`<h3 class="svelte-13nvtxg">`);
	$.push_element($$renderer, "h3", 171, 8);
	$$renderer.push(`${$.escape(heading)}</h3>`);
	$.pop_element();
	$$renderer.push(` <p class="svelte-13nvtxg">`);
	$.push_element($$renderer, "p", 172, 8);
	$$renderer.push(`${$.escape(body)}</p>`);
	$.pop_element();
	$$renderer.push(` `);
	badge($$renderer, "new", "primary");
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
export const BENCHMARK_KIND = "compiler";
export const MODULE_SCALE = 3;
export function moduleLabel(name) {
	return `${BENCHMARK_KIND}:${name}`;
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const propsId = $.props_id($$renderer);
		var $$store_subs;
		let { title = "Default Title", count = 0, items = [], config = {}, multiplier = 2, visible = false, $$slots, $$events, ...rest } = $$props;
		let state = "";
		let counter = 0;
		let rawData = {
			x: 1,
			y: 2
		};
		let checked = false;
		let group = [];
		let volume = .5;
		let selected = "opt-0";
		let inputEl;
		let componentRef;
		let dynamicEl;
		let metrics = writable([
			1,
			2,
			3
		]);
		let labelStore = writable("ready");
		let show;
		counter = 10;
		let doubled = $.derived(() => count * multiplier);
		let computed = $.derived(() => {
			return items.length * multiplier + counter;
		});
		let moduleSummary = $.derived(() => moduleLabel(title) + ":" + MODULE_SCALE);
		let storeSummary = $.derived(() => $.store_get($$store_subs ??= {}, "$metrics", metrics).length + ":" + $.store_get($$store_subs ??= {}, "$labelStore", labelStore));
		let snapshot = rawData;
		let tracking = false;
		console.log("$inspect(", counter, doubled(), ")");
		const APP_VERSION = "1.0.0";
		function formatTitle(prefix) {
			return prefix + ": " + title;
		}
		function addMetric() {
			$.store_set(metrics, [...$.store_get($$store_subs ??= {}, "$metrics", metrics), counter]);
			$.store_set(labelStore, title);
		}
		function action(node, arg) {
			return { destroy() {} };
		}
		function handleClick(e) {
			counter++;
		}
		function getHandler() {
			return handleClick;
		}
		function handleError(error) {
			console.error(error);
		}
		let promise = Promise.resolve(42);
		$.prevent_snippet_stringification(metricSummary);
		function metricSummary($$renderer, { label, values = [counter], meta: { id = propsId } = {} }) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<section class="summary svelte-13nvtxg"${$.attr("data-id", id)}>`);
			$.push_element($$renderer, "section", 178, 4);
			$$renderer.push(`<h4 class="svelte-13nvtxg">`);
			$.push_element($$renderer, "h4", 179, 8);
			$$renderer.push(`${$.escape(label)}</h4>`);
			$.pop_element();
			$$renderer.push(` <!--[-->`);
			const each_array = $.ensure_array_like(values);
			for (let index = 0, $$length = each_array.length; index < $$length; index++) {
				let value = each_array[index];
				$$renderer.push(`<span class="svelte-13nvtxg">`);
				$.push_element($$renderer, "span", 181, 12);
				$$renderer.push(`${$.escape(index)}: ${$.escape(value)}</span>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]--></section>`);
			$.pop_element();
		}
		const $$tag = state ? "div" : "span";
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$.prevent_snippet_stringification(failed);
		function failed($$renderer, error) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<p class="svelte-13nvtxg">`);
			$.push_element($$renderer, "p", 294, 12);
			$$renderer.push(`Error in chunk 0: ${$.escape(error.message)}</p>`);
			$.pop_element();
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>${$.escape(title)} - Benchmark</title>`);
			});
			$$renderer.push(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/>`);
			$.push_element($$renderer, "meta", 155, 4);
			$.pop_element();
			$$renderer.push(` <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`);
			$.push_element($$renderer, "link", 156, 4);
			$.pop_element();
		});
		$$renderer.push(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0">`);
		console.log({
			counter,
			state
		});
		debugger;
		$.push_element($$renderer, "div", 186, 0);
		$$renderer.push(`Chunk 0: Lorem ${$.escape(state)} + ${$.escape(state)} = Ipsum; <p class="svelte-13nvtxg">`);
		$.push_element($$renderer, "p", 188, 4);
		$$renderer.push(`Props: title=${$.escape(title)}, count=${$.escape(count)}, doubled=${$.escape(doubled())}, computed=${$.escape(computed())}</p>`);
		$.pop_element();
		$$renderer.push(` <p class="svelte-13nvtxg">`);
		$.push_element($$renderer, "p", 189, 4);
		$$renderer.push(`Module: ${$.escape(moduleSummary())} | Store: ${$.escape(storeSummary())} | Label: ${$.escape($.store_get($$store_subs ??= {}, "$labelStore", labelStore))}</p>`);
		$.pop_element();
		$$renderer.push(` ${$.html("<b>raw html chunk 0</b>")} <div${$.attr_class($.clsx({
			active: checked,
			big: counter > 10
		}), "svelte-13nvtxg", {
			"state": state,
			"staticly": true,
			"invinsible": invinsible,
			"reactive": counter
		})}${$.attr_style("", {
			color: state,
			"font-size": "14px",
			opacity: counter / 100,
			"--custom": "value-0"
		})}>`);
		$.push_element($$renderer, "div", 194, 4);
		$$renderer.push(`Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. `);
		if (state) {
			$$renderer.push("<!--[0-->");
			const localLen = state.length;
			$$renderer.push(`<span${$.attr("title", `${$.stringify(title)}: ${$.stringify(doubled())}`)} empty=""${$.attr("state", state)}${$.attr("counter", counter)}${$.attr("count", count)} class="svelte-13nvtxg">`);
			$.push_element($$renderer, "span", 215, 12);
			$$renderer.push(`Duis aute irure dolor: ${$.escape(localLen)}. Chunk 0.</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div class="svelte-13nvtxg">`);
			$.push_element($$renderer, "div", 219, 12);
			$$renderer.push(`<input${$.attr("title", title)}${$.attr("state", state)}${$.attr("value", count)} class="svelte-13nvtxg"/>`);
			$.push_element($$renderer, "input", 220, 16);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(` `);
			if (counter > 30) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1${$.attr("state", state)} class="svelte-13nvtxg">`);
				$.push_element($$renderer, "h1", 224, 16);
				$$renderer.push(`Lorem ipsum dolor sit amet. Chunk 0.</h1>`);
				$.pop_element();
			} else if (counter == 100) {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`Lorem ipsum dolor sit amet. Chunk 0.`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2 class="svelte-13nvtxg">`);
				$.push_element($$renderer, "h2", 230, 16);
				$$renderer.push(`EMPTY</h2>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
		$$renderer.push(` <!---->`);
		{
			$$renderer.push(`<p class="svelte-13nvtxg">`);
			$.push_element($$renderer, "p", 236, 8);
			$$renderer.push(`Keyed content chunk 0: ${$.escape(counter)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!----> <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array_1.length; idx < $$length; idx++) {
			let item = each_array_1[idx];
			const itemLabel = `${idx}:${item.name}`;
			$$renderer.push(`<p${$.attributes({
				...rest,
				"data-index": `chunk-0-${$.stringify(idx)}`
			}, "svelte-13nvtxg")}>`);
			$.push_element($$renderer, "p", 241, 8);
			$$renderer.push(`${$.escape(itemLabel)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_2 = $.ensure_array_like(items);
		for (let $$index_2 = 0, $$length = each_array_2.length; $$index_2 < $$length; $$index_2++) {
			$$renderer.push(`<span class="item-less svelte-13nvtxg">`);
			$.push_element($$renderer, "span", 245, 8);
			$$renderer.push(`Repeated shell chunk 0</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> `);
		$.await($$renderer, promise, () => {
			$$renderer.push(`<p class="svelte-13nvtxg">`);
			$.push_element($$renderer, "p", 249, 8);
			$$renderer.push(`Loading chunk 0...</p>`);
			$.pop_element();
		}, (value) => {
			$$renderer.push(`<p class="svelte-13nvtxg">`);
			$.push_element($$renderer, "p", 251, 8);
			$$renderer.push(`Resolved: ${$.escape(value)}</p>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]--> `);
		$.await($$renderer, promise, () => {}, (quickValue) => {
			$$renderer.push(`<p class="svelte-13nvtxg">`);
			$.push_element($$renderer, "p", 257, 8);
			$$renderer.push(`Quick resolved: ${$.escape(quickValue)}</p>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]--> <input${$.attr("value", state)} class="svelte-13nvtxg"/>`);
		$.push_element($$renderer, "input", 260, 4);
		$.pop_element();
		$$renderer.push(` <textarea class="svelte-13nvtxg">`);
		$.push_element($$renderer, "textarea", 261, 4);
		const $$body = $.escape(state);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.select({
			value: selected,
			class: ""
		}, ($$renderer) => {
			$$renderer.option({
				value: "opt-0",
				class: ""
			}, ($$renderer) => {
				$.push_element($$renderer, "option", 263, 8);
				$$renderer.push(`Zero`);
				$.pop_element();
			}, "svelte-13nvtxg");
			$$renderer.option({
				value: "opt-1",
				class: ""
			}, ($$renderer) => {
				$.push_element($$renderer, "option", 264, 8);
				$$renderer.push(`One`);
				$.pop_element();
			}, "svelte-13nvtxg");
		}, "svelte-13nvtxg");
		$$renderer.push(` <input type="checkbox"${$.attr("checked", checked, true)} class="svelte-13nvtxg"/>`);
		$.push_element($$renderer, "input", 266, 4);
		$.pop_element();
		$$renderer.push(` <input type="radio"${$.attr("checked", group === "opt-0", true)} value="opt-0" class="svelte-13nvtxg"/>`);
		$.push_element($$renderer, "input", 267, 4);
		$.pop_element();
		$$renderer.push(` <div contenteditable="" class="svelte-13nvtxg">`);
		$.push_element($$renderer, "div", 268, 4);
		if (state) {
			$$renderer.push(`${state}`);
		} else {
			$$renderer.push(`editable`);
		}
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` <video class="svelte-13nvtxg">`);
		$.push_element($$renderer, "video", 269, 4);
		$$renderer.push(`</video>`);
		$.pop_element();
		$$renderer.push(` <div class="svelte-13nvtxg">`);
		$.push_element($$renderer, "div", 271, 4);
		$$renderer.push(`action target</div>`);
		$.pop_element();
		$$renderer.push(` <div class="svelte-13nvtxg">`);
		$.push_element($$renderer, "div", 272, 4);
		$$renderer.push(`transition target</div>`);
		$.pop_element();
		$$renderer.push(` <div class="svelte-13nvtxg">`);
		$.push_element($$renderer, "div", 273, 4);
		$$renderer.push(`in/out target</div>`);
		$.pop_element();
		$$renderer.push(` `);
		$.push_element($$renderer, $$tag, 274, 4);
		$.element($$renderer, $$tag, () => {
			$$renderer.push(` class="dynamic-0 svelte-13nvtxg"`);
		}, () => {
			$$renderer.push(`Dynamic element chunk 0: ${$.escape(title)}`);
		});
		$.pop_element();
		$$renderer.push(` `);
		ChildComponent($$renderer, {
			title,
			onclick: getHandler(),
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<strong class="svelte-13nvtxg">`);
				$.push_element($$renderer, "strong", 279, 8);
				$$renderer.push(`Inline child chunk 0: ${$.escape(title)}</strong>`);
				$.pop_element();
			}),
			$$slots: {
				default: true,
				footer: ($$renderer) => {
					$$renderer.push(`<div slot="footer" class="svelte-13nvtxg">`);
					$.push_element($$renderer, "div", 280, 8);
					$$renderer.push(`Footer chunk 0: ${$.escape(counter)}</div>`);
					$.pop_element();
				}
			}
		});
		$$renderer.push(`<!----> `);
		badge($$renderer, "chunk-0", "secondary");
		$$renderer.push(`<!----> `);
		card($$renderer, title, "Content for chunk 0");
		$$renderer.push(`<!----> `);
		metricSummary($$renderer, {
			label: title,
			values: [
				count,
				doubled(),
				counter
			],
			meta: { id: propsId }
		});
		$$renderer.push(`<!----> `);
		show?.($$renderer);
		$$renderer.push(`<!----> <button class="svelte-13nvtxg">`);
		$.push_element($$renderer, "button", 288, 4);
		$$renderer.push(`Update store</button>`);
		$.pop_element();
		$$renderer.push(` <p class="svelte-13nvtxg">`);
		$.push_element($$renderer, "p", 289, 4);
		$$renderer.push(`Metric count: ${$.escape($.store_get($$store_subs ??= {}, "$metrics", metrics).length)}</p>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$$renderer.push(`<p class="svelte-13nvtxg">`);
				$.push_element($$renderer, "p", 292, 8);
				$$renderer.push(`Boundary chunk 0: ${$.escape(title)}</p>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`</div>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			config,
			visible,
			APP_VERSION,
			formatTitle
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
