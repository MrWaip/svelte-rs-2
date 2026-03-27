import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click</button>`), App[$.FILENAME], [[28, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	let data = $.tag($.state(null), "data");
	const handleArrow = () => {
		return $.trace(() => "handleArrow ((unknown):5:21)", () => {
			$.update(count);
		});
	};
	async function fetchData() {
		return await $.trace(() => "fetchData ((unknown):10:1)", async () => {
			$.set(data, (await $.track_reactivity_loss(fetch("/api")))(), true);
		});
	}
	foo(() => {
		return $.trace(() => "foo(...) ((unknown):15:5)", () => {
			$.update(count);
		});
	});
	const obj = { handler() {
		return $.trace(() => "handler ((unknown):21:9)", () => {
			$.update(count);
		});
	} };
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $.trace(() => "trace ((unknown):28:17)", () => {
			$.update(count);
		});
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
