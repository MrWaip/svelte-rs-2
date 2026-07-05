import * as $ from "svelte/internal/server";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
import { onMount } from "svelte";
function badge($$renderer, text, variant) {
	$$renderer.push(`<span${$.attr_class("badge", void 0, {
		"primary": variant === "primary",
		"secondary": variant === "secondary"
	})}>${$.escape(text)}</span>`);
}
function card($$renderer, heading, body) {
	$$renderer.push(`<div class="card"><h3>${$.escape(heading)}</h3> <p>${$.escape(body)}</p> `);
	badge($$renderer, "new", "primary");
	$$renderer.push(`<!----></div>`);
}
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { title, theme = "light", editable = void 0, config = {}, multiplier = 2, $$slots, $$events, ...extras } = $$props;
		let count = 0;
		let query = "";
		let state = "";
		let counter = 0;
		let items = [
			"Задачи",
			"Settings",
			"🌞 Profile"
		];
		counter = 10;
		count += 1;
		let doubled = $.derived(() => count * multiplier);
		const VERSION = "2.0";
		const APP_VERSION = "1.0.0";
		function reset() {
			count = 0;
		}
		function formatTitle(prefix) {
			return prefix + ": " + title;
		}
		function increment() {
			count++;
		}
		function row($$renderer, item) {
			$$renderer.push(`<li>${$.escape(item)} — ${$.escape(count)}</li>`);
		}
		$$renderer.push(`<header${$.attributes({
			id: "top",
			"data-theme": theme,
			title: `Dashboard: ${$.stringify(title)}`,
			...extras
		})}><h1>${$.escape(title)} 🚀</h1> <input${$.attr("value", query)}/> <button>${$.escape(count)}</button></header> `);
		if (count > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<section><p>Результат: ${$.escape(count)} for ${$.escape(query)}</p> <!--[-->`);
			const each_array = $.ensure_array_like(items);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				row($$renderer, item);
				$$renderer.push(`<!----> <div class="entry"${$.attr("data-q", `q: ${$.stringify(query)}`)}>${$.escape(item)}</div>`);
			}
			$$renderer.push(`<!--]--></section>`);
		} else if (editable) {
			$$renderer.push("<!--[1-->");
			Panel($$renderer, {
				label: "empty",
				count,
				children: ($$renderer) => {
					$$renderer.push(`<p>Nothing here yet</p> `);
					Panel($$renderer, {
						label: "empty",
						count,
						children: ($$renderer) => {
							$$renderer.push(`<!---->Title <p>Nothing here yet</p>`);
						},
						$$slots: { default: true }
					});
					$$renderer.push(`<!---->`);
				},
				$$slots: { default: true }
			});
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<noscript>Enable JS</noscript> <p>${$.escape(count = 0)}</p>`);
		}
		$$renderer.push(`<!--]--> `);
		Panel($$renderer, {
			count,
			label: title
		});
		$$renderer.push(`<!----> <div>Chunk 0: Lorem ${$.escape(state)} + ${$.escape(state)} = Ipsum; <p>Props: title=${$.escape(title)}, count=${$.escape(count)}, doubled=${$.escape(doubled())}</p> <div${$.attr_class("", void 0, {
			"state": state,
			"staticly": true,
			"invinsible": invinsible,
			"reactive": counter
		})}>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. `);
		if (state) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span${$.attr("title", `${$.stringify(title)}: ${$.stringify(doubled())}`)} empty=""${$.attr("state", state)}${$.attr("counter", counter)}${$.attr("count", count)}>Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div><input${$.attr("title", title)}${$.attr("state", state)}${$.attr("value", count)}/></div> `);
			if (counter > 30) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1${$.attr("state", state)}>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.</h1>`);
			} else if (counter == 100) {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`Lorem ipsum dolor sit amet. Chunk 0.`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2>EMPTY</h2>`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div> <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let item = each_array_1[$$index_1];
			$$renderer.push(`<p${$.attributes({
				...extras,
				"data-index": "chunk-0"
			})}>${$.escape(item)}</p>`);
		}
		$$renderer.push(`<!--]--> <input${$.attr("value", state)}/> `);
		badge($$renderer, "chunk-0", "secondary");
		$$renderer.push(`<!----> `);
		card($$renderer, title, "Content for chunk 0");
		$$renderer.push(`<!----></div>`);
		$.bind_props($$props, {
			editable,
			config,
			VERSION,
			APP_VERSION,
			reset,
			formatTitle
		});
	});
}
