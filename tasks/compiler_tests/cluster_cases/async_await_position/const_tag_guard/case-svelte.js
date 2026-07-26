import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve({ value });
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 16, () => [1], $.index, ($$anchor, item) => {
		let current;
		var promises = $.run([async () => current = (await $.save($.async_derived(async () => (await $.save(delay($.get(x))))().value)))()]);
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(current) ?? ""}${item ?? ""}`), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
