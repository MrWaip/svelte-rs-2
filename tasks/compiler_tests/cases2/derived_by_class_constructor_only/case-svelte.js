import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Box {
		#total;
		get total() {
			return $.get(this.#total);
		}
		set total(value) {
			$.set(this.#total, value);
		}
		#width = $.state(2);
		get width() {
			return $.get(this.#width);
		}
		set width(value) {
			$.set(this.#width, value, true);
		}
		#height = $.state(3);
		get height() {
			return $.get(this.#height);
		}
		set height(value) {
			$.set(this.#height, value, true);
		}
		#area = $.derived(() => this.width * this.height);
		get area() {
			return $.get(this.#area);
		}
		set area(value) {
			$.set(this.#area, value);
		}
		constructor() {
			this.#total = $.derived(() => this.area + 1);
		}
	}
	let box = new Box();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${box.area ?? ""},${box.total ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
