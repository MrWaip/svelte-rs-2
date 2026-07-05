App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Panel from "./Panel.svelte";
import { formatDate } from "./utils.js";
import { onMount } from "svelte";
$.prevent_snippet_stringification(badge);
function badge($$renderer, text, variant) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span${$.attr_class("badge", void 0, {
		"primary": variant === "primary",
		"secondary": variant === "secondary"
	})}>`);
	$.push_element($$renderer, "span", 51, 4);
	$$renderer.push(`${$.escape(text)}</span>`);
	$.pop_element();
}
$.prevent_snippet_stringification(card);
function card($$renderer, heading, body) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div class="card">`);
	$.push_element($$renderer, "div", 57, 4);
	$$renderer.push(`<h3>`);
	$.push_element($$renderer, "h3", 58, 8);
	$$renderer.push(`${$.escape(heading)}</h3>`);
	$.pop_element();
	$$renderer.push(` <p>`);
	$.push_element($$renderer, "p", 59, 8);
	$$renderer.push(`${$.escape(body)}</p>`);
	$.pop_element();
	$$renderer.push(` `);
	badge($$renderer, "new", "primary");
	$$renderer.push(`<!----></div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
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
		$.prevent_snippet_stringification(row);
		function row($$renderer, item) {
			$.validate_snippet_args($$renderer);
			$$renderer.push(`<li>`);
			$.push_element($$renderer, "li", 47, 4);
			$$renderer.push(`${$.escape(item)} — ${$.escape(count)}</li>`);
			$.pop_element();
		}
		$$renderer.push(`<header${$.attributes({
			id: "top",
			"data-theme": theme,
			title: `Dashboard: ${$.stringify(title)}`,
			...extras
		})}>`);
		$.push_element($$renderer, "header", 64, 0);
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 65, 4);
		$$renderer.push(`${$.escape(title)} 🚀</h1>`);
		$.pop_element();
		$$renderer.push(` <input${$.attr("value", query)}/>`);
		$.push_element($$renderer, "input", 66, 4);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 67, 4);
		$$renderer.push(`${$.escape(count)}</button>`);
		$.pop_element();
		$$renderer.push(`</header>`);
		$.pop_element();
		$$renderer.push(` `);
		if (count > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<section>`);
			$.push_element($$renderer, "section", 71, 4);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 72, 8);
			$$renderer.push(`Результат: ${$.escape(count)} for ${$.escape(query)}</p>`);
			$.pop_element();
			$$renderer.push(` <!--[-->`);
			const each_array = $.ensure_array_like(items);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				row($$renderer, item);
				$$renderer.push(`<!----> <div class="entry"${$.attr("data-q", `q: ${$.stringify(query)}`)}>`);
				$.push_element($$renderer, "div", 77, 12);
				$$renderer.push(`${$.escape(item)}</div>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]--></section>`);
			$.pop_element();
		} else if (editable) {
			$$renderer.push("<!--[1-->");
			Panel($$renderer, {
				label: "empty",
				count,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 84, 8);
					$$renderer.push(`Nothing here yet</p>`);
					$.pop_element();
					$$renderer.push(` `);
					Panel($$renderer, {
						label: "empty",
						count,
						children: $.prevent_snippet_stringification(($$renderer) => {
							$$renderer.push(`<!---->Title <p>`);
							$.push_element($$renderer, "p", 88, 12);
							$$renderer.push(`Nothing here yet</p>`);
							$.pop_element();
						}),
						$$slots: { default: true }
					});
					$$renderer.push(`<!---->`);
				}),
				$$slots: { default: true }
			});
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<noscript>`);
			$.push_element($$renderer, "noscript", 92, 4);
			$$renderer.push(`Enable JS</noscript>`);
			$.pop_element();
			$$renderer.push(` <p>`);
			$.push_element($$renderer, "p", 93, 4);
			$$renderer.push(`${$.escape(count = 0)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> `);
		Panel($$renderer, {
			count,
			label: title
		});
		$$renderer.push(`<!----> <div>`);
		$.push_element($$renderer, "div", 98, 0);
		$$renderer.push(`Chunk 0: Lorem ${$.escape(state)} + ${$.escape(state)} = Ipsum; <p>`);
		$.push_element($$renderer, "p", 100, 4);
		$$renderer.push(`Props: title=${$.escape(title)}, count=${$.escape(count)}, doubled=${$.escape(doubled())}</p>`);
		$.pop_element();
		$$renderer.push(` <div${$.attr_class("", void 0, {
			"state": state,
			"staticly": true,
			"invinsible": invinsible,
			"reactive": counter
		})}>`);
		$.push_element($$renderer, "div", 101, 4);
		$$renderer.push(`Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. `);
		if (state) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span${$.attr("title", `${$.stringify(title)}: ${$.stringify(doubled())}`)} empty=""${$.attr("state", state)}${$.attr("counter", counter)}${$.attr("count", count)}>`);
			$.push_element($$renderer, "span", 113, 12);
			$$renderer.push(`Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 120, 12);
			$$renderer.push(`<input${$.attr("title", title)}${$.attr("state", state)}${$.attr("value", count)}/>`);
			$.push_element($$renderer, "input", 121, 16);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
			$$renderer.push(` `);
			if (counter > 30) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<h1${$.attr("state", state)}>`);
				$.push_element($$renderer, "h1", 125, 16);
				$$renderer.push(`Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.</h1>`);
				$.pop_element();
			} else if (counter == 100) {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`Lorem ipsum dolor sit amet. Chunk 0.`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 133, 16);
				$$renderer.push(`EMPTY</h2>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let item = each_array_1[$$index_1];
			$$renderer.push(`<p${$.attributes({
				...extras,
				"data-index": "chunk-0"
			})}>`);
			$.push_element($$renderer, "p", 139, 8);
			$$renderer.push(`${$.escape(item)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--> <input${$.attr("value", state)}/>`);
		$.push_element($$renderer, "input", 142, 4);
		$.pop_element();
		$$renderer.push(` `);
		badge($$renderer, "chunk-0", "secondary");
		$$renderer.push(`<!----> `);
		card($$renderer, title, "Content for chunk 0");
		$$renderer.push(`<!----></div>`);
		$.pop_element();
		$.bind_props($$props, {
			editable,
			config,
			VERSION,
			APP_VERSION,
			reset,
			formatTitle
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
