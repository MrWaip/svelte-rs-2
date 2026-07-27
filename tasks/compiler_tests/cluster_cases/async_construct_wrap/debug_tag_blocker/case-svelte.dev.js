import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let data;
		var promises = $.run([async () => data = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve("works")))())))(), "data")]);
		$.template_effect(() => {
			console.log({ data: $.snapshot($.get(data)) });
			debugger;
		}, [], [], [promises[0]]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
