<template>
  <div class="warning-tip" :style="{fontSize:size==='small'?'13px':''}">
    <svg-icon icon-class="info" class="mr5" :color="'#eca013'"></svg-icon>
    <slot>
    </slot>
  </div>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "WarningTip",
  components: {SvgIcon},
  props: {
    size: String
  }
}
</script>

<style scoped>
.warning-tip {
  background: #fdf0de;
  border-radius: 5px;
  padding: 5px 10px;
  line-height: 20px;
  font-size: 14px;
  color: #fc8702;
}
</style>