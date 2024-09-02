<script>
    export let selected = false;
    export let large = false;
    export let style = "";
    export let onClick = () => {
    };
</script>
<button class={selected ? "selected":""} style={`${style} ${large ? "width:28px; height:28px;":""}`}
        on:click={onClick}>
  <slot></slot>
</button>

<style lang="scss">

  button {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #7fb8ed;
    border: none;
    border-radius: 3px;
    padding: 2px 5px;
    box-sizing: border-box;
    cursor: pointer;
    color: white;

    &:hover {
      background: #6694be;
    }
  }

  button.selected {
    background: #669ed5;
    color: white;
  }
</style>
