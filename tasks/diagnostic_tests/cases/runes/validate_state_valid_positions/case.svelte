<script>
let x = $state(0);
let y = $state.raw('hello');

class Foo {
    count = $state(0);
    constructor() {
        this.name = $state('');
    }
}
</script>
