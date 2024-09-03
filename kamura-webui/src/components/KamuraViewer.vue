<template>
  <el-container class="kamura-viewer" style="height: 80vh">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu :default-openeds="['2']">
          <el-sub-menu index="1">
            <template #title>
              Arches
            </template>
            <el-menu-item-group>
              <el-menu-item index="1-1">simple</el-menu-item>
              <el-menu-item index="1-2">alu8</el-menu-item>
              <el-menu-item index="1-3">no-lsu2</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

          <el-sub-menu index="2">
            <template #title>
              Tools
            </template>
            <el-menu-item-group>
              <el-menu-item index="2-1" @click="setMode('view')">View</el-menu-item>
              <el-menu-item index="2-2" @click="setMode('remove')">Remove</el-menu-item>
              <el-menu-item index="2-3">Add</el-menu-item>
              <el-menu-item index="2-4" @click="restore">Undo</el-menu-item>
              <el-menu-item index="2-5">Redo</el-menu-item>
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
import cytoscape from 'cytoscape';
import {parseYaml, parseBindings} from '@/utils/funcs';
import {options} from '@/utils/layout';


export default {
  name: 'KamuraViewer',
  data() {
    return {
      cy: null,
      cyElements: [],
      removedNodes: [],
    };
  },
  async mounted() {
    const response = await fetch('/simple.yaml');
    const yamlText = await response.text();
    const data = parseYaml(yamlText);
    if (data) {
      const bindingTopology = data['top.extension.topo_extensions']['binding_topology'];
      this.initCytoscape(bindingTopology);
    }
  },
  methods: {
    initCytoscape(bindingTopology) {
      this.cyElements = parseBindings(bindingTopology);

      this.cy = this.createCyto();

      this.setMode('view');
    },
    createCyto() {
      return cytoscape({
        container: this.$refs.cyRef,
        elements: this.cyElements,
        style: [
          {
            selector: 'node',
            style: {
              'background-color': '#666',
              'label': 'data(label)',
              'text-valign': 'center',
              'color': '#000',
              'width': '40px',
              'height': '40px',
            }
          },
          {
            selector: '.subgraph',
            style: {
              'background-color': '#ddd',
              'shape': 'rectangle',
              'padding': '10px',
              'border-color': '#333',
              'border-width': '2px'
            }
          },
          {
            selector: 'edge',
            style: {
              'line-color': '#111',
              'target-arrow-color': '#000',
              'target-arrow-shape': 'triangle'
            }
          }
        ],
        layout: options.preset
      });
    },
    setMode(mode) {
      switch (mode) {
        case 'remove':
          this.cy.$('node').on('tap', (e) => {
            const ele = e.target;
            this.removedNodes.push(ele.remove());
          });
          break;
        case 'view':
          this.cy.$('node').on('tap', (e) => {
            e
          });
          break;
      }
    },
    restore() {
      let top = this.removedNodes.pop();
      if (top) {
        // console.log(top);
        top.restore();
      }
    },
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.cyRef {
  border-style: solid;
  border-width: thin;
  border-color: gray;
}
</style>
