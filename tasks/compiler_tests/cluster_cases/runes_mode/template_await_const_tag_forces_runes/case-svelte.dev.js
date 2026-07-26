import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>inc</button>`, 1), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	const promise = Promise.resolve(1);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let value;
			var promises = $.run([async () => value = $.tag((await $.save($.async_derived(async () => (await $.save(promise))())))(), "value")]);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(value) ?? ""} ${count ?? ""}`), void 0, void 0, [promises[0]]);
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (count >= 0) $$render(consequent);
		}), "if", App, 6, 0);
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
