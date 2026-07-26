import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> `, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	function pick(first, second) {
		return first + second;
	}
	function wrap(object) {
		return object.value;
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(($0, $1, $2, $3, $4) => $.set_text(text, ` ${$0 ?? ""}
${$1 ?? ""}
${$2 ?? ""}
${$3 ?? ""}
${$4 ?? ""}`), void 0, [
		async () => pick(1, await delay($.get(x))),
		async () => $.get(x) > 0 ? await delay($.get(x)) : 0,
		async () => `value ${await delay($.get(x))}`,
		async () => (0, await delay($.get(x))),
		async () => wrap({
			index: 1,
			value: await delay($.get(x))
		})
	]);
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
