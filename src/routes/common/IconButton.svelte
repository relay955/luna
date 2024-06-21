<script>
    export let selected = false;
    export let large = false;
    export let style = "";
    export let onClick = () => {};
</script>
<button class={selected ? "selected":""} style={`${style} ${large ? "width:28px; height:28px;":""}`} on:click={onClick}>
    <slot></slot>
</button>

<style lang="scss">
    button{
        width: 20px;
        height: 20px;
        background: white;
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