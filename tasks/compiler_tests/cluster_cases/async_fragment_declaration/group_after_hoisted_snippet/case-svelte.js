import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1> <!>`, 1);
var root_1 = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let name = "world";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const greet = ($$anchor) => {
			let greeting;
			var promises_1 = $.run([async () => greeting = (await $.save($.async_derived(async () => (await $.save(`Hello, ${name}!`))())))()]);
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
					var promises_2 = $.run([() => promises[0].promise, async () => length = (await $.save($.async_derived(async () => (await $.save($.get(number)))())))()]);
					var fragment_2 = $.comment();
					var node_2 = $.first_child(fragment_2);
					$.async(node_2, [promises_2[1]], void 0, (node_2) => {
						$.each(node_2, 17, () => ({ length: $.get(length) }), $.index, ($$anchor, $$item, index) => {
							let i;
							var promises_3 = $.run([async () => i = (await $.save($.async_derived(async () => (await $.save(index))())))()]);
							$.next();
							var text_2 = $.text();
							$.template_effect(() => $.set_text(text_2, $.get(i)), void 0, void 0, [promises_3[0]]);
							$.append($$anchor, text_2);
						});
					});
					$.append($$anchor, fragment_2);
				};
				$.if(node_1, ($$render) => {
					if ($.get(number) > 4 && $.get(after_async) && $.get(greeting)) $$render(consequent);
				});
			});
			$.template_effect(() => {
				$.set_text(text, $.get(greeting));
				$.set_text(text_1, ` ${$.get(number) ?? ""} `);
			}, void 0, void 0, [promises_1[0], promises[0]]);
			$.append($$anchor, fragment_1);
		};
		const sync = $.derived(() => "sync");
		let number;
		let after_async;
		let computed_const;
		var promises = $.run([
			async () => number = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))(),
			() => after_async = $.derived(() => $.get(number) + 1),
			async () => computed_const = (await $.save($.async_derived(async () => {
				const { length, 0: first } = (await $.save("01234"))();
				return {
					length,
					first
				};
			})))()
		]);
		var fragment_4 = root_1();
		var node_3 = $.first_child(fragment_4);
		greet(node_3);
		var text_3 = $.sibling(node_3);
		var node_4 = $.sibling(text_3);
		{
			var consequent_1 = ($$anchor) => {
				let double;
				var promises_4 = $.run([() => promises[0].promise, () => double = $.derived(() => $.get(number) * 2)]);
				var text_4 = $.text();
				$.template_effect(() => $.set_text(text_4, $.get(double)), void 0, void 0, [promises_4[1]]);
				$.append($$anchor, text_4);
			};
			$.if(node_4, ($$render) => {
				if ($.get(sync)) $$render(consequent_1);
			});
		}
		$.template_effect(() => $.set_text(text_3, ` ${$.get(number) ?? ""} ${$.get(sync) ?? ""} ${$.get(after_async) ?? ""} ${$.get(computed_const).length ?? ""} ${$.get(computed_const).first ?? ""} `), void 0, void 0, [
			promises[0],
			promises[1],
			promises[2]
		]);
		$.append($$anchor, fragment_4);
	});
	$.append($$anchor, fragment);
}
