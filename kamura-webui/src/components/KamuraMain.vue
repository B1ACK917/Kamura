<template>
  <el-container class="kamura-main" style="height: 500px">
    <el-header style="text-align: right; font-size: 12px">
      <el-tabs v-model="activeName" class="demo-tabs">
        <el-tab-pane label="Kamura-Viewer" name="first">
          <kamura-viewer></kamura-viewer>
        </el-tab-pane>
        <el-tab-pane label="Kamura-Runner" name="second">
          <kamura-runner></kamura-runner>
          /
        </el-tab-pane>
        <el-tab-pane label="Settings" name="third">Settings</el-tab-pane>
      </el-tabs>
    </el-header>
  </el-container>
</template>


<script>
import KamuraViewer from "@/components/KamuraViewer.vue";
import KamuraRunner from "@/components/KamuraRunner.vue";
import {ref} from "vue";

export default {
  name: 'KamuraMain',
  components: {KamuraRunner, KamuraViewer},
  data() {
    return {
      activeName: ref('first')
    };
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.el-menu--horizontal > .el-menu-item:nth-child(1) {
  margin-right: auto;
}
</style>
