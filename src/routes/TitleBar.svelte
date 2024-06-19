<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import Space from "./Space.svelte";
  import logo from "../libs/icon.png";
  const onClickMinimize = () => appWindow.minimize();
  const onClickMaximize = () => appWindow.toggleMaximize();
  const onClickClose = () => appWindow.close();
</script>

<div data-tauri-drag-region class="titlebar">
  <img class="logo" src={logo} alt="luna_logo" />
  <div class="title">Luna</div>
  <Space />
  <button
    class="titlebar-button"
    id="titlebar-minimize"
    on:click={onClickMinimize}
  >
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </button>
  <button
    class="titlebar-button"
    id="titlebar-maximize"
    on:click={onClickMaximize}
  >
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
    />
  </button>
  <button class="titlebar-button" id="titlebar-close" on:click={onClickClose}>
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </button>
</div>

<style lang="scss">
  .logo{
    width: 20px;
    height: 20px;
    margin-right: 10px;
  }
  .title {
    font-size: 14px;
    font-weight: bold;
    color: white;
  }
  .titlebar {
    height: 30px;
    background: #669ed5;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    align-items: center;
    padding-left: 10px;
    box-shadow: #c2c2c2 0px 0px 10px;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    border: none;
    background: #669ed5;
    img{
      filter: invert(1);
    }
  }
  .titlebar-button:hover {
    background: #669ed5;
  }
</style>
