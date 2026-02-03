<template>
  <div class="info-tip" :style="{fontSize:size==='small'?'13px':''}">
    <svg-icon icon-class="info" class="mr5" :color="'#888888'"></svg-icon>
    <slot>
    </slot>
  </div>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "InfoTip",
  components: {SvgIcon},
  props: {
    size: String
  }
}
</script>

<style scoped>
.info-tip {
  background: #f5f7fa;
  border-radius: 5px;
  padding: 8px 10px;
  line-height: 24px;
  font-size: 14px;
  color: #7e7e7e;
}
</style>