App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let time = new Date();
		let hours = $.derived(() => time.getHours());
		let minutes = $.derived(() => time.getMinutes());
		let seconds = $.derived(() => time.getSeconds());
		onMount(() => {
			const interval = setInterval(() => {
				time = new Date();
			}, 1e3);
			return () => {
				clearInterval(interval);
			};
		});
		$$renderer.push(`<svg viewBox="-50 -50 100 100" class="svelte-1kjtqer">`);
		$.push_element($$renderer, "svg", 21, 0);
		$$renderer.push(`<circle class="clock-face svelte-1kjtqer" r="48">`);
		$.push_element($$renderer, "circle", 22, 1);
		$$renderer.push(`</circle>`);
		$.pop_element();
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like([
			0,
			5,
			10,
			15,
			20,
			25,
			30,
			35,
			40,
			45,
			50,
			55
		]);
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let minute = each_array[$$index_1];
			$$renderer.push(`<line class="major svelte-1kjtqer" y1="35" y2="45"${$.attr("transform", `rotate(${$.stringify(30 * minute)})`)}>`);
			$.push_element($$renderer, "line", 25, 2);
			$$renderer.push(`</line>`);
			$.pop_element();
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like([
				1,
				2,
				3,
				4
			]);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let offset = each_array_1[$$index];
				$$renderer.push(`<line class="minor svelte-1kjtqer" y1="42" y2="45"${$.attr("transform", `rotate(${$.stringify(6 * (minute + offset))})`)}>`);
				$.push_element($$renderer, "line", 28, 3);
				$$renderer.push(`</line>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--><line class="hour svelte-1kjtqer" y1="2" y2="-20"${$.attr("transform", `rotate(${$.stringify(30 * hours() + minutes() / 2)})`)}>`);
		$.push_element($$renderer, "line", 32, 1);
		$$renderer.push(`</line>`);
		$.pop_element();
		$$renderer.push(`<line class="minute svelte-1kjtqer" y1="4" y2="-30"${$.attr("transform", `rotate(${$.stringify(6 * minutes() + seconds() / 10)})`)}>`);
		$.push_element($$renderer, "line", 33, 1);
		$$renderer.push(`</line>`);
		$.pop_element();
		$$renderer.push(`<g${$.attr("transform", `rotate(${$.stringify(6 * seconds())})`)}>`);
		$.push_element($$renderer, "g", 35, 1);
		$$renderer.push(`<line class="second svelte-1kjtqer" y1="10" y2="-38">`);
		$.push_element($$renderer, "line", 36, 2);
		$$renderer.push(`</line>`);
		$.pop_element();
		$$renderer.push(`<line class="second-counterweight svelte-1kjtqer" y1="10" y2="2">`);
		$.push_element($$renderer, "line", 37, 2);
		$$renderer.push(`</line>`);
		$.pop_element();
		$$renderer.push(`</g>`);
		$.pop_element();
		$$renderer.push(`</svg>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
