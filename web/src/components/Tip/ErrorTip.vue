<template>
  <div class="error-tip" :style="{fontSize:size==='small'?'13px':''}">
    <svg-icon icon-class="info" class="mr5" :color="'#cc1c1c'"></svg-icon>
    <slot>
    </slot>
  </div>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "ErrorTip",
  components: {SvgIcon},
  props: {
    size: String
  }
}
</script>

<style scoped>
.error-tip {
  background: #fef0f0;
  border-radius: 5px;
  padding: 5px 10px;
  line-height: 20px;
  font-size: 14px;
  color: #f56c6c;
}
</style>