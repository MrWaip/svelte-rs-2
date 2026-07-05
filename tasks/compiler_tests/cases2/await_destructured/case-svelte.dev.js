App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const promise = fetch("/api");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { name, age } = $.get($$source);
			return {
				name,
				age
			};
		});
		var name = $.derived(() => $.get($$value).name);
		var age = $.derived(() => $.get($$value).age);
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} is ${$.get(age) ?? ""}`));
		$.append($$anchor, p);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
