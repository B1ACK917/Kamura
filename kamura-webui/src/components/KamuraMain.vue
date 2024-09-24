<template>
  <el-container class="kamura-main" style="height: 80vh">
    <el-header style="font-size: 12px">
      <el-tabs v-model="activeName" class="demo-tabs">
        <el-tab-pane label="Kamura-Viewer" name="first">
          <kamura-viewer @updateSharedValues="updateSharedValues"></kamura-viewer>
        </el-tab-pane>
        <el-tab-pane label="Kamura-Runner" name="second">
          <kamura-runner v-if="activeName === 'second'" :sharedSelectedArch="sharedSelectedArch"></kamura-runner>
        </el-tab-pane>
        <el-tab-pane label="Settings" name="third">
          <kamura-settings v-if="activeName === 'third'" :sharedSelectedArch="sharedSelectedArch"></kamura-settings>
        </el-tab-pane>
      </el-tabs>
    </el-header>
  </el-container>
</template>


<script>
import KamuraViewer from "@/components/KamuraViewer.vue";
import KamuraRunner from "@/components/KamuraRunner.vue";
import KamuraSettings from "@/components/KamuraSettings.vue";

export default {
  name: 'KamuraMain',
  components: {KamuraSettings, KamuraRunner, KamuraViewer},
  data() {
    return {
      activeName: 'first',
      sharedSelectedArch: null
    };
  },
  methods: {
    updateSharedValues(newValue) {
      console.log(newValue);
      this.sharedSelectedArch = newValue;
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
