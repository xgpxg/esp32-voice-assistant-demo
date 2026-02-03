<template>
  <div class="common-layout">
    <el-container>
      <el-aside :width="menuWidth+'px'">
        <Menu :collapse="collapse"></Menu>
      </el-aside>
      <el-main>
        <el-header>
          <Navbar @collapse="switchCollapse"></Navbar>
        </el-header>
        <AppMain></AppMain>
      </el-main>
    </el-container>
    <el-backtop target=".el-main" :bottom="160" :visibility-height="70"
                :right="20"></el-backtop>
  </div>
</template>


<script>

import Navbar from "./components/Navbar.vue";
import AppMain from "./components/AppMain.vue";
import Menu from "./components/Menu.vue";
import AppBar from "./components/AppBar.vue";

export default {
  components: {AppBar, Menu, AppMain, Navbar},
  data() {
    return {
      menuWidth: 170,
      top: 0,
      showLogin: false,
      collapse: false,
    }
  },
  provide() {
    return {
      scrollTop: this.scrollTop,
      scrollTo: this.scrollTo,
      leftMenuWidth: this.getMenuWidth,
      isEnterprise: this.isEnterprise,
    }
  },
  mounted() {
    PubSub.subscribe('NEED_LOGIN', (msg, data) => {
      this.showLogin = true
    })
  },
  computed: {
    isEnterprise() {
      const productType = localStorage.getItem('_PRODUCT_TYPE') || ''
      return productType === 'enterprise'
    }
  },
  methods: {
    getKey(route) {
      return route.fullPath;
    },
    switchCollapse(isCollapse) {
      this.collapse = isCollapse
      if (isCollapse) {
        this.menuWidth = 65
      } else {
        this.menuWidth = 170
      }
    },
    /**
     * 滚动事件
     * @param scrollTop
     */
    scroll({scrollTop}) {
      this.top = scrollTop
    },
    /**
     * 设置滚动距离顶部的位置
     * @param top
     */
    scrollTo(top) {
      this.$refs['scrollbar'].setScrollTop(top)
    },
    /**
     * 获取滚动距离顶部的位置
     * @returns {number}
     */
    scrollTop() {
      return this.top
    },
    getMenuWidth() {
      return this.menuWidth
    },
    // isEnterprise() {
    //   const productType = localStorage.getItem('_PRODUCT_TYPE') || ''
    //   return productType === 'enterprise'
    // }
  }
}
</script>

<style scoped lang="scss">
.el-header {
  padding: 0;
}

.el-main {
  margin: 0 auto;
  padding: 0;
}
</style>