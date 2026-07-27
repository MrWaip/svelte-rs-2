import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> `, 1), App[$.FILENAME], [[17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	function pick(first, second) {
		return first + second;
	}
	function wrap(object) {
		return object.value;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(($0, $1, $2, $3, $4) => $.set_text(text, ` ${$0 ?? ""}
${$1 ?? ""}
${$2 ?? ""}
${$3 ?? ""}
${$4 ?? ""}`), void 0, [
		async () => pick(1, (await $.track_reactivity_loss(delay($.get(x))))()),
		async () => $.get(x) > 0 ? (await $.track_reactivity_loss(delay($.get(x))))() : 0,
		async () => `value ${(await $.track_reactivity_loss(delay($.get(x))))()}`,
		async () => (0, (await $.track_reactivity_loss(delay($.get(x))))()),
		async () => wrap({
			index: 1,
			value: (await $.track_reactivity_loss(delay($.get(x))))()
		})
	]);
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
