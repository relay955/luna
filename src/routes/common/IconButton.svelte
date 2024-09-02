<script>

    export let selected = false;
    export let large = false;
    export let style = "";
    export let onClick = () => {
    };
</script>
<button class={selected ? "selected":""} style={`${style} ${large ? "width:28px; height:28px;":""}`}
        on:click={onClick}>
  <span style={large ? "width:20px; height:20px;" : "width:15px; height:15px;"}>
    <slot></slot>
  </span>
</button>

<style lang="scss">

  button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    background: white;
    border: none;
    border-radius: 3px;
    padding: 0;
    box-sizing: border-box;
    cursor: pointer;

    &:hover {
      background: #e7e7e7;
    }
  }

  button.selected {
    background: #669ed5;
    color: white;
  }
</style>
