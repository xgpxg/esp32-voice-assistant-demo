<template>
  <svg-icon icon-class="copy" @click="copy(text)"
            class="cursor-pointer"></svg-icon>
</template>

<script>


import SvgIcon from "@components/SvgIcon/index.vue";
import {copyText} from 'vue3-clipboard'

export default {
  name: "copy-text",
  components: {SvgIcon},
  props: {
    text: String | Number
  },
  data() {
    return {}
  },
  created() {
    this.init()
  },
  methods: {
    init() {

    },
    copy(text) {
      copyText(text, undefined, () => {
        this.$message.success('已复制到剪贴板')
      })
    }
  }
}
</script>


<style scoped lang="scss">

</style>