import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
const pending = ($$anchor) => {
	$.next();
	var text = $.text("loading");
	$.append($$anchor, text);
};
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get pending() {
		return pending;
	} }, ($$anchor) => {
		let data;
		var promises = $.run([async () => data = (await $.save($.async_derived(async () => (await $.save(Promise.resolve("d")))())))()]);
		$.next();
		var text_1 = $.text();
		$.template_effect(() => $.set_text(text_1, $.get(data)), void 0, void 0, [promises[0]]);
		$.append($$anchor, text_1);
	});
	$.append($$anchor, fragment);
}
