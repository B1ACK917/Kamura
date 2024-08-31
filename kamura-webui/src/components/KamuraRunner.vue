<template>
  <el-container class="kamura-runner" style="height: 800px">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu>
          <el-sub-menu index="1">
            <template #title>
              Traces
            </template>
            <el-menu-item-group>
              <el-menu-item index="1-1">hello</el-menu-item>
              <el-menu-item index="1-2">dhrystone</el-menu-item>
              <el-menu-item index="1-3">quicksort</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
          <el-sub-menu index="2">
            <template #title>
              Runners
            </template>
            <el-menu-item-group>
              <el-menu-item index="2-1">20240831-simple-hello</el-menu-item>
              <el-menu-item index="2-2">20240831-simple-dhrystone</el-menu-item>
              <el-menu-item index="2-3">20240831-simple-quicksort</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main>
      <div ref="cyRef" style="width: 99%; height: 600px; text-align: left" class="cyRef"></div>
    </el-main>

  </el-container>
</template>


<script>
export default {
  name: 'KamuraRunner',
  data() {
    return {};
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
