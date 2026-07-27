import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1> </h1> <!>`, 1), App[$.FILENAME], [[13, 2]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const greet = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			let greeting;
			var promises_1 = $.run([async () => greeting = $.tag((await $.save($.async_derived(async () => (await $.save(`Hello, ${name}!`))())))(), "greeting")]);
			var fragment_1 = root();
			var h1 = $.first_child(fragment_1);
			var text = $.child(h1, true);
			$.reset(h1);
			var text_1 = $.sibling(h1);
			var node_1 = $.sibling(text_1);
			$.async(node_1, [
				promises[0],
				promises[1],
				promises_1[0]
			], void 0, (node_1) => {
				var consequent = ($$anchor) => {
					let length;
					var promises_2 = $.run([() => promises[0].promise, async () => length = $.tag((await $.save($.async_derived(async () => (await $.save($.get(number)))())))(), "length")]);
					var fragment_2 = $.comment();
					var node_2 = $.first_child(fragment_2);
					$.async(node_2, [promises_2[1]], void 0, (node_2) => {
						$.add_svelte_meta(() => $.each(node_2, 17, () => ({ length: $.get(length) }), $.index, ($$anchor, $$item, index) => {
							let i;
							var promises_3 = $.run([async () => i = $.tag((await $.save($.async_derived(async () => (await $.save(index))())))(), "i")]);
							$.next();
							var text_2 = $.text();
							$.template_effect(() => $.set_text(text_2, $.get(i)), void 0, void 0, [promises_3[0]]);
							$.append($$anchor, text_2);
						}), "each", App, 17, 3);
					});
					$.append($$anchor, fragment_2);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if ($.get(number) > 4 && $.get(after_async) && $.get(greeting)) $$render(consequent);
				}), "if", App, 15, 2);
			});
			$.template_effect(() => {
				$.set_text(text, $.get(greeting));
				$.set_text(text_1, ` ${$.get(number) ?? ""} `);
			}, void 0, void 0, [promises_1[0], promises[0]]);
			$.append($$anchor, fragment_1);
		});
		const sync = $.tag($.derived(() => "sync"), "sync");
		$.get(sync);
		let number;
		let after_async;
		let computed_const;
		var promises = $.run([
			async () => number = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))(), "number"),
			() => after_async = $.tag($.derived(() => $.get(number) + 1), "after_async"),
			async () => computed_const = $.tag((await $.save($.async_derived(async () => {
				const { length, 0: first } = (await $.save("01234"))();
				return {
					length,
					first
				};
			})))(), "[@const]")
		]);
		var fragment_4 = root_1();
		var node_3 = $.first_child(fragment_4);
		$.add_svelte_meta(() => greet(node_3), "render", App, 24, 1);
		var text_3 = $.sibling(node_3);
		var node_4 = $.sibling(text_3);
		{
			var consequent_1 = ($$anchor) => {
				let double;
				var promises_4 = $.run([() => promises[0].promise, () => double = $.tag($.derived(() => $.get(number) * 2), "double")]);
				var text_4 = $.text();
				$.template_effect(() => $.set_text(text_4, $.get(double)), void 0, void 0, [promises_4[1]]);
				$.append($$anchor, text_4);
			};
			$.add_svelte_meta(() => $.if(node_4, ($$render) => {
				if ($.get(sync)) $$render(consequent_1);
			}), "if", App, 27, 1);
		}
		$.template_effect(() => $.set_text(text_3, ` ${$.get(number) ?? ""} ${$.get(sync) ?? ""} ${$.get(after_async) ?? ""} ${$.get(computed_const).length ?? ""} ${$.get(computed_const).first ?? ""} `), void 0, void 0, [
			promises[0],
			promises[1],
			promises[2]
		]);
		$.append($$anchor, fragment_4);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
