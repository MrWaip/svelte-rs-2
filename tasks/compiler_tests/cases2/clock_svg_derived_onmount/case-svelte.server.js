import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
export default function App($$renderer, $$props) {
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
		$$renderer.push(`<svg viewBox="-50 -50 100 100" class="svelte-1kjtqer"><circle class="clock-face svelte-1kjtqer" r="48"></circle><!--[-->`);
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
			$$renderer.push(`<line class="major svelte-1kjtqer" y1="35" y2="45"${$.attr("transform", `rotate(${$.stringify(30 * minute)})`)}></line><!--[-->`);
			const each_array_1 = $.ensure_array_like([
				1,
				2,
				3,
				4
			]);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let offset = each_array_1[$$index];
				$$renderer.push(`<line class="minor svelte-1kjtqer" y1="42" y2="45"${$.attr("transform", `rotate(${$.stringify(6 * (minute + offset))})`)}></line>`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]--><line class="hour svelte-1kjtqer" y1="2" y2="-20"${$.attr("transform", `rotate(${$.stringify(30 * hours() + minutes() / 2)})`)}></line><line class="minute svelte-1kjtqer" y1="4" y2="-30"${$.attr("transform", `rotate(${$.stringify(6 * minutes() + seconds() / 10)})`)}></line><g${$.attr("transform", `rotate(${$.stringify(6 * seconds())})`)}><line class="second svelte-1kjtqer" y1="10" y2="-38"></line><line class="second-counterweight svelte-1kjtqer" y1="10" y2="2"></line></g></svg>`);
	});
}
