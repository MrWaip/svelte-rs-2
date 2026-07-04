App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
var root = $.add_locations($.from_svg(`<line class="minor svelte-1kjtqer" y1="42" y2="45"></line>`), App[$.FILENAME], [[28, 3]]);
var root_1 = $.add_locations($.from_svg(`<line class="major svelte-1kjtqer" y1="35" y2="45"></line><!>`, 1), App[$.FILENAME], [[25, 2]]);
var root_2 = $.add_locations($.from_svg(`<svg viewBox="-50 -50 100 100" class="svelte-1kjtqer"><circle class="clock-face svelte-1kjtqer" r="48"></circle><!><line class="hour svelte-1kjtqer" y1="2" y2="-20"></line><line class="minute svelte-1kjtqer" y1="4" y2="-30"></line><g><line class="second svelte-1kjtqer" y1="10" y2="-38"></line><line class="second-counterweight svelte-1kjtqer" y1="10" y2="2"></line></g></svg>`), App[$.FILENAME], [[
	21,
	0,
	[
		[22, 1],
		[32, 1],
		[33, 1],
		[
			35,
			1,
			[[36, 2], [37, 2]]
		]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let time = $.tag($.state($.proxy(new Date())), "time");
	let hours = $.tag($.derived(() => $.get(time).getHours()), "hours");
	let minutes = $.tag($.derived(() => $.get(time).getMinutes()), "minutes");
	let seconds = $.tag($.derived(() => $.get(time).getSeconds()), "seconds");
	onMount(() => {
		const interval = setInterval(() => {
			$.set(time, new Date(), true);
		}, 1e3);
		return () => {
			clearInterval(interval);
		};
	});
	var $$exports = { ...$.legacy_api() };
	var svg = root_2();
	var node = $.sibling($.child(svg));
	$.add_svelte_meta(() => $.each(node, 16, () => [
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
	], $.index, ($$anchor, minute) => {
		var fragment = root_1();
		var line = $.first_child(fragment);
		var node_1 = $.sibling(line);
		$.add_svelte_meta(() => $.each(node_1, 16, () => [
			1,
			2,
			3,
			4
		], $.index, ($$anchor, offset) => {
			var line_1 = root();
			$.template_effect(() => $.set_attribute(line_1, "transform", `rotate(${6 * (minute + offset)})`));
			$.append($$anchor, line_1);
		}), "each", App, 27, 2);
		$.template_effect(() => $.set_attribute(line, "transform", `rotate(${30 * minute})`));
		$.append($$anchor, fragment);
	}), "each", App, 24, 1);
	var line_2 = $.sibling(node);
	var line_3 = $.sibling(line_2);
	var g = $.sibling(line_3);
	$.reset(svg);
	$.template_effect(() => {
		$.set_attribute(line_2, "transform", `rotate(${30 * $.get(hours) + $.get(minutes) / 2})`);
		$.set_attribute(line_3, "transform", `rotate(${6 * $.get(minutes) + $.get(seconds) / 10})`);
		$.set_attribute(g, "transform", `rotate(${6 * $.get(seconds)})`);
	});
	$.append($$anchor, svg);
	return $.pop($$exports);
}
