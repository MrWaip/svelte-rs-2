import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
function badge($$renderer, text, variant) {
	$$renderer.push(`<span${$.attr_class("badge svelte-13nvtxg", void 0, {
		"primary": variant === "primary",
		"secondary": variant === "secondary"
	})}>${$.escape(text)}</span>`);
}
function card($$renderer, heading, body) {
	$$renderer.push(`<div class="card svelte-13nvtxg"><h3 class="svelte-13nvtxg">${$.escape(heading)}</h3> <p class="svelte-13nvtxg">${$.escape(body)}</p> `);
	badge($$renderer, "new", "primary");
	$$renderer.push(`<!----></div>`);
}
export const BENCHMARK_KIND = "compiler";
export const MODULE_SCALE = 3;
export function moduleLabel(name) {
	return `${BENCHMARK_KIND}:${name}`;
}
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { title = "Default Title", count = 0, items = [], config = $.store_get($$store_subs ??= {}, "$bindable", bindable)({}), multiplier = 2, visible = $.store_get($$store_subs ??= {}, "$bindable", bindable)(false), ...rest } = $.store_get($$store_subs ??= {}, "$props", props)();
		const propsId = $.store_get($$store_subs ??= {}, "$props", props).id();
		let state = $.store_get($$store_subs ??= {}, "$state", state)("");
		let counter = $.store_get($$store_subs ??= {}, "$state", state)(0);
		let rawData = $.store_get($$store_subs ??= {}, "$state", state).raw({
			x: 1,
			y: 2
		});
		let checked = $.store_get($$store_subs ??= {}, "$state", state)(false);
		let group = $.store_get($$store_subs ??= {}, "$state", state)([]);
		let volume = $.store_get($$store_subs ??= {}, "$state", state)(.5);
		let selected = $.store_get($$store_subs ??= {}, "$state", state)("opt-0");
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
		let doubled = $.store_get($$store_subs ??= {}, "$derived", derived)(count * multiplier);
		let computed = $.store_get($$store_subs ??= {}, "$derived", derived).by(() => {
			return items.length * multiplier + counter;
		});
		let moduleSummary = $.store_get($$store_subs ??= {}, "$derived", derived)(moduleLabel(title) + ":" + MODULE_SCALE);
		let storeSummary = $.store_get($$store_subs ??= {}, "$derived", derived)($.store_get($$store_subs ??= {}, "$metrics", metrics).length + ":" + $.store_get($$store_subs ??= {}, "$labelStore", labelStore));
		let snapshot = $.store_get($$store_subs ??= {}, "$state", state).snapshot(rawData);
		$.store_get($$store_subs ??= {}, "$effect", effect)(() => {
			console.log("Title:", title, "Count:", count);
		});
		$.store_get($$store_subs ??= {}, "$effect", effect).pre(() => {
			console.log("Pre effect:", counter);
		});
		let tracking = $.store_get($$store_subs ??= {}, "$effect", effect).tracking();
		$.store_get($$store_subs ??= {}, "$inspect", inspect)(counter, doubled);
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
		function metricSummary($$renderer, { label, values = [counter], meta: { id = propsId } = {} }) {
			$$renderer.push(`<section class="summary svelte-13nvtxg"${$.attr("data-id", id)}><h4 class="svelte-13nvtxg">${$.escape(label)}</h4> <!--[-->`);
			const each_array = $.ensure_array_like(values);
			for (let index = 0, $$length = each_array.length; index < $$length; index++) {
				let value = each_array[index];
				$$renderer.push(`<span class="svelte-13nvtxg">${$.escape(index)}: ${$.escape(value)}</span>`);
			}
			$$renderer.push(`<!--]--></section>`);
		}
		function failed($$renderer, error) {
			$$renderer.push(`<p class="svelte-13nvtxg">Error in chunk 0: ${$.escape(error.message)}</p>`);
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>${$.escape(title)} - Benchmark</title>`);
			});
			$$renderer.push(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/> <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`);
		});
		$$renderer.push(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0">`);
		console.log({
			counter,
			state
		});
		debugger;
		$$renderer.push(`Chunk 0: Lorem ${$.escape(state)} + ${$.escape(state)} = Ipsum; <p class="svelte-13nvtxg">Props: title=${$.escape(title)}, count=${$.escape(count)}, doubled=${$.escape(doubled)}, computed=${$.escape(computed)}</p> <p class="svelte-13nvtxg">Module: ${$.escape(moduleSummary)} | Store: ${$.escape(storeSummary)} | Label: ${$.escape($.store_get($$store_subs ??= {}, "$labelStore", labelStore))}</p> ${$.html("<b>raw html chunk 0</b>")} <div${$.attr_class($.clsx({
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
		})}>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. `);
		if (state) {
			$$renderer.push("<!--[0-->");
			const localLen = state.length;
			$$renderer.push(`<span${$.attr("title", `${$.stringify(title)}: ${$.stringify(doubled)}`)} empty=""${$.attr("state", state)}${$.attr("counter", counter)}${$.attr("count", count)} class="svelte-13nvtxg">Duis aute irure dolor: ${$.escape(localLen)}. Chunk 0.</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div class="svelte-13nvtxg"><input${$.attr("title", title)}${$.attr("state", state)}${$.attr("value", count)} class="svelte-13nvtxg"/></div> `);
			if (counter > 30) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1${$.attr("state", state)} class="svelte-13nvtxg">Lorem ipsum dolor sit amet. Chunk 0.</h1>`);
			} else if (counter == 100) {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`Lorem ipsum dolor sit amet. Chunk 0.`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2 class="svelte-13nvtxg">EMPTY</h2>`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div> <!---->`);
		{
			$$renderer.push(`<p class="svelte-13nvtxg">Keyed content chunk 0: ${$.escape(counter)}</p>`);
		}
		$$renderer.push(`<!----> <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array_1.length; idx < $$length; idx++) {
			let item = each_array_1[idx];
			const itemLabel = `${idx}:${item.name}`;
			$$renderer.push(`<p${$.attributes({
				...rest,
				"data-index": `chunk-0-${$.stringify(idx)}`
			}, "svelte-13nvtxg")}>${$.escape(itemLabel)}</p>`);
		}
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_2 = $.ensure_array_like(items);
		for (let $$index_2 = 0, $$length = each_array_2.length; $$index_2 < $$length; $$index_2++) {
			$$renderer.push(`<span class="item-less svelte-13nvtxg">Repeated shell chunk 0</span>`);
		}
		$$renderer.push(`<!--]--> `);
		$.await($$renderer, promise, () => {
			$$renderer.push(`<p class="svelte-13nvtxg">Loading chunk 0...</p>`);
		}, (value) => {
			$$renderer.push(`<p class="svelte-13nvtxg">Resolved: ${$.escape(value)}</p>`);
		});
		$$renderer.push(`<!--]--> `);
		$.await($$renderer, promise, () => {}, (quickValue) => {
			$$renderer.push(`<p class="svelte-13nvtxg">Quick resolved: ${$.escape(quickValue)}</p>`);
		});
		$$renderer.push(`<!--]--> <input${$.attr("value", state)} class="svelte-13nvtxg"/> <textarea class="svelte-13nvtxg">`);
		const $$body = $.escape(state);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea> `);
		$$renderer.select({
			value: selected,
			class: ""
		}, ($$renderer) => {
			$$renderer.option({
				value: "opt-0",
				class: ""
			}, ($$renderer) => {
				$$renderer.push(`Zero`);
			}, "svelte-13nvtxg");
			$$renderer.option({
				value: "opt-1",
				class: ""
			}, ($$renderer) => {
				$$renderer.push(`One`);
			}, "svelte-13nvtxg");
		}, "svelte-13nvtxg");
		$$renderer.push(` <input type="checkbox"${$.attr("checked", checked, true)} class="svelte-13nvtxg"/> <input type="radio"${$.attr("checked", group === "opt-0", true)} value="opt-0" class="svelte-13nvtxg"/> <div contenteditable="" class="svelte-13nvtxg">`);
		if (state) {
			$$renderer.push(`${state}`);
		} else {
			$$renderer.push(`editable`);
		}
		$$renderer.push(`</div> <video class="svelte-13nvtxg"></video> <div class="svelte-13nvtxg">action target</div> <div class="svelte-13nvtxg">transition target</div> <div class="svelte-13nvtxg">in/out target</div> `);
		$.element($$renderer, state ? "div" : "span", () => {
			$$renderer.push(` class="dynamic-0 svelte-13nvtxg"`);
		}, () => {
			$$renderer.push(`Dynamic element chunk 0: ${$.escape(title)}`);
		});
		$$renderer.push(` `);
		ChildComponent($$renderer, {
			title,
			onclick: getHandler(),
			children: ($$renderer) => {
				$$renderer.push(`<strong class="svelte-13nvtxg">Inline child chunk 0: ${$.escape(title)}</strong>`);
			},
			$$slots: {
				default: true,
				footer: ($$renderer) => {
					$$renderer.push(`<div slot="footer" class="svelte-13nvtxg">Footer chunk 0: ${$.escape(counter)}</div>`);
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
				doubled,
				counter
			],
			meta: { id: propsId }
		});
		$$renderer.push(`<!----> `);
		show?.($$renderer);
		$$renderer.push(`<!----> <button class="svelte-13nvtxg">Update store</button> <p class="svelte-13nvtxg">Metric count: ${$.escape($.store_get($$store_subs ??= {}, "$metrics", metrics).length)}</p> `);
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$$renderer.push(`<p class="svelte-13nvtxg">Boundary chunk 0: ${$.escape(title)}</p>`);
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`</div>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			APP_VERSION,
			formatTitle
		});
	});
}
