<script>
	class Todo {
		constructor() {
			this.items = $state([]);
			this.name = $state('');
		}
		add() {
			this.items.push(this.name);
		}
	}
	let todo = new Todo();
</script>

<p>{todo.items.length} - {todo.name}</p>
