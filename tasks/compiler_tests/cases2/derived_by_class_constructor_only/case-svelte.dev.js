App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Box {
		#total;
		get total() {
			return $.get(this.#total);
		}
		set total(value) {
			$.set(this.#total, value);
		}
		#width = $.tag($.state(2), "Box.width");
		get width() {
			return $.get(this.#width);
		}
		set width(value) {
			$.set(this.#width, value, true);
		}
		#height = $.tag($.state(3), "Box.height");
		get height() {
			return $.get(this.#height);
		}
		set height(value) {
			$.set(this.#height, value, true);
		}
		#area = $.tag($.derived(() => this.width * this.height), "Box.area");
		get area() {
			return $.get(this.#area);
		}
		set area(value) {
			$.set(this.#area, value);
		}
		constructor() {
			this.#total = $.tag($.derived(() => this.area + 1), "Box.total");
		}
	}
	let box = new Box();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${box.area ?? ""},${box.total ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
