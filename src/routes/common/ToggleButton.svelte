<script>
    export let selected = false;
    export let style = "";
</script>
<button class={selected ? "selected":""} style={style}>
    <slot></slot>
</button>

<style lang="scss">
    button{
        width: 20px;
        height: 20px;
        background: #f7f7f7;
        border: none;
        border-radius: 3px;
        padding: 4px;
        box-sizing: border-box;
        cursor: pointer;
        &:hover{
            background: #e7e7e7;
        }
    }

    button.selected{
        background: #669ed5;
        color: white;
    }
</style>