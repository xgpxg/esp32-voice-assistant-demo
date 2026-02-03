<template>
  <div class="navbar flex-space-between">
    <div class="flex-v fill-width">
      <Hamburger class="hamburger-container"
                 @toggleClick="toggleSideBar" :is-active="collapse"></Hamburger>
                  <Breadcrumb class="breadcrumb-container"></Breadcrumb>
<!--      <AppBar></AppBar>-->
    </div>

    <div class="right-menu flex-v">
      <el-popover width="500px" trigger="click" @show="" placement="bottom-end" v-if="U.isDev()">
        <template #reference>
          <el-icon class="cursor-pointer" style="margin-top: -2px;margin-right: 10px">
            <svg-icon icon-class="app"></svg-icon>
          </el-icon>
        </template>
      </el-popover>
      <el-popover width="500px" trigger="click" @show="this.$refs.messageRef?.loadMessages()" placement="bottom-end">
        <template #reference>
          <el-badge :value="unreadCount" class="ml20 cursor-pointer" :show-zero="false">
            <el-icon>
              <Bell/>
            </el-icon>
          </el-badge>
        </template>
        <Message ref="messageRef"></Message>
      </el-popover>
      <el-link
          target="_blank"
          class="nav-icon-button"
          @click="toHome"
          :underline="false">
        <el-dropdown trigger="click" placement="bottom-end">
          <div class="avatar-wrapper">
            <el-icon class="user-avatar">
              <User></User>
            </el-icon>
          </div>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="$refs['updatePassword'].show()">
                修改密码
              </el-dropdown-item>
            </el-dropdown-menu>
            <el-dropdown-menu>
              <el-dropdown-item @click="logout">
                登出
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-link>
    </div>
  </div>
  <update-password ref="updatePassword"></update-password>
</template>

<script>
import SvgIcon from "@components/SvgIcon/index.vue";
import Hamburger from "./Hamburger.vue";
import Breadcrumb from "./Breadcrumb.vue";
import UpdatePassword from "../../views/login/update-password.vue";
import AppBar from "./AppBar.vue";
import Message from "./Message.vue";
import {U} from "../../utils/util";

export default {
  components: {
    Message,
    AppBar,
    UpdatePassword,
    Hamburger,
    SvgIcon,
    Breadcrumb
  },
  props: {},
  inject: ['isEnterprise'],
  data() {
    return {
      collapse: false,
      unreadCount: 0,
    }
  },
  computed: {
    U() {
      return U
    },
    user() {
      return this.$store.state.user
    },
  },
  created() {
    this.loadUserInfo()
    this.loadUnreadCount()
    setInterval(() => this.loadUnreadCount(), 10 * 1000)
  },
  methods: {
    loadUserInfo() {
      /* this.R.get('user/center', {}, {repeatable: true}).then(res => {
         this.$store.commit('user/setAvatar', res.data.base_info.avatar)
         this.$store.commit('user/setNickname', res.data.base_info.nickname)
         this.$store.commit('user/setEmail', res.data.base_info.email)
         this.$store.commit('user/setOther', res.data.other)
       })*/
    },
    logout() {
      this.R.postJson('/api/user/logout', {}, {repeatable: true}).then(res => {
        if (res.code === 0) {
          this.$router.replace({name: 'Login'})
        }
      })
    },
    toggleSideBar() {
      this.collapse = !this.collapse
      this.$emit('collapse', this.collapse)
    },
    toMessage() {
      this.$router.push({name: 'Alert'})
    },
    toHome() {
      this.$router.push({name: 'Index'})
    },
    loadUnreadCount() {
      this.R.postJson('/api/message/count/unread').then(res => {
        this.unreadCount = res.data.info + res.data.warn + res.data.error
      })
    }
  }
}
</script>

<style lang="scss" scoped>
.navbar {
  overflow: hidden;
  position: relative;
  background: #ffffff;
  z-index: 100;

  .left-menu {
    height: 100%;
    align-items: center;

    .logo-container {
      display: flex;
      align-items: center;
      margin-right: 48px;

      .logo {
        height: 36px;
        width: 36px;
        margin-right: 12px;
      }

      .logo-text {
        font-size: 20px;
        font-weight: 600;
        color: #ffffff;
        letter-spacing: 0.5px;
        background: var(--el-color-primary);
        padding: 5px 10px;
        border-radius: 5px;
      }
    }

    .nav-menu {
      display: flex;
      height: 100%;

      .nav-link {
        height: 100%;
        display: flex;
        align-items: center;
        padding: 0 22px;
        font-size: 15px;
        color: #595959;
        font-weight: 500;
        transition: all 0.3s;
        position: relative;

        &:hover {
          color: #262626;
        }

        &.active {
          color: #1890ff;
          font-weight: 500;

          &::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 3px;
            background-color: #1890ff;
          }
        }
      }
    }
  }

  .right-menu {
    height: 100%;
    align-items: center;
    margin-right: 20px;

    .nav-icon-button {
      margin: 0 6px 0 20px;
      background: transparent;
      border: none;
      color: #595959;
      width: 40px;
      height: 40px;

      &:hover {
        color: #1890ff;
        background: #f0f7ff;
      }

      .el-icon {
        font-size: 18px;
      }
    }

    .avatar-container {
      .avatar-wrapper {
        position: relative;
        display: flex;
        align-items: center;

        .user-avatar {
          cursor: pointer;
          width: 36px;
          height: 36px;
        }

        .default-avatar {
          background: transparent;
          width: 36px;
          height: 36px;
        }
      }
    }
  }
}

.dropdown-menu {
  padding: 16px 20px;
  border-radius: 6px;

  .user-info {
    padding-bottom: 16px;
    border-bottom: 1px solid #f0f0f0;

    .user-name {
      font-size: 16px;
      font-weight: 500;
      color: #262626;
      margin-bottom: 4px;
    }

    .user-role {
      font-size: 13px;
      color: #8c8c8c;
    }
  }

  .dropdown-actions {
    padding-top: 16px;
    display: flex;
    flex-direction: column;

    .dropdown-button {
      margin-bottom: 10px;
      border-radius: 4px;

      &:last-child {
        margin-bottom: 0;
      }

      &:hover {
        border-color: #1890ff;
        color: #1890ff;
      }
    }

    .logout-btn:hover {
      border-color: #ff4d4f;
      color: #ff4d4f;
      background-color: #fff;
    }
  }
}

:deep(.el-badge__content) {
  height: 14px;
  line-height: 14px;
  padding: 0 4px;
  font-size: 10px;
  min-width: 12px;
}

</style>