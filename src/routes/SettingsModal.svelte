<script>
    import Modal from "../complex-comp/Modal.svelte";
    import Input from "../basic-comp/Input.svelte";
    import Button from "../basic-comp/Button.svelte";

    export let isOpen = false;
    export let onClose = () => {};

    $:isOpen, {

    }
</script>

<Modal title="설정" isOpen={isOpen} onClose={onClose}>
  <div>
    <div class="header">Protection Mode</div>
    <div style="display: flex; align-items: center">
      <span class="sub-header">파일명 복호화 </span>
      <input type="checkbox" style="width: 15px; height: 15px; margin-top: 5px"/>
    </div>
    <div class="description">임시 암호화 해제시, 파일명도 복호화합니다.</div>
    <div class="sub-header">복호화 임시파일 위치</div>
    <div style="width:100%; display: flex; margin-top: 5px">
      <Input value="1" style="flex: 1; width: 0; margin-right: 10px" />
      <Button>찾아보기</Button>
    </div>
    <Button style="margin-top: 20px; width: 100%;">저장</Button>
  </div>
</Modal>

<style lang="scss">
  .header {
    font-size: 16px;
    font-weight: bold;
    margin-bottom: 2px;
  }

  .sub-header {
    font-size: 14px;
    color: #656565;
  }

  .description{
    font-size: 12px;
    color: #656565;
    margin-bottom: 10px;
  }
</style>
