import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let data;
		var promises = $.run([async () => data = (await $.save($.async_derived(async () => (await $.save(Promise.resolve("works")))())))()]);
		$.template_effect(() => {
			console.log({ data: $.snapshot($.get(data)) });
			debugger;
		}, [], [], [promises[0]]);
	});
	$.append($$anchor, fragment);
}
